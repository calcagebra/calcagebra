use std::{collections::HashMap, path::Path};

use inkwell::{
    builder::Builder,
    context::Context,
    module::Module,
    passes::PassBuilderOptions,
    targets::{CodeModel, FileType, InitializationConfig, RelocMode, Target, TargetMachine},
    types::BasicMetadataTypeEnum,
    values::{
        BasicMetadataValueEnum, BasicValue, FloatValue, FunctionValue, GlobalValue, PointerValue,
    },
    AddressSpace, FloatPredicate,
};

use crate::{
    ast::{self, Ast, Expression},
    token::Token,
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Builder error: {0}")]
    Builer(#[from] inkwell::builder::BuilderError),

    #[error("Verification error:\n{0}")]
    Verification(String),

    #[error("LLVM Error:\n{0}")]
    Other(String),
}

#[derive(Debug)]
pub struct Compiler<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,

    variables: HashMap<String, Variable<'ctx>>,
    functions: HashMap<String, Function<'ctx>>,

    current_function: Option<FunctionValue<'ctx>>,

    printf: (FunctionValue<'ctx>, GlobalValue<'ctx>),
    scanf: (FunctionValue<'ctx>, GlobalValue<'ctx>),
}

impl<'ctx> Compiler<'ctx> {
    pub fn new(context: &'ctx Context, module_name: &str) -> Self {
        let module = context.create_module(module_name);
        let builder = context.create_builder();
        let printf = Compiler::create_printf(context, &module);
        let scanf = Compiler::create_scanf(context, &module);

        Self {
            context,
            module,
            builder,
            printf,
            scanf,
            current_function: None,
            variables: HashMap::default(),
            functions: HashMap::default(),
        }
    }

    pub fn dump_to_stderr(&self) {
        self.module.print_to_stderr();
    }

    pub fn verify(&self) -> Result<(), Error> {
        self.module
            .verify()
            .map_err(|llvm_str| Error::Verification(llvm_str.to_string()))
    }

    pub fn create_target_machine() -> Result<TargetMachine, Error> {
        Target::initialize_all(&InitializationConfig::default());

        let target_triple = TargetMachine::get_default_triple();

        let target =
            Target::from_triple(&target_triple).map_err(|err| Error::Other(err.to_string()))?;

        let target_machine = target
            .create_target_machine(
                &target_triple,
                "generic",
                "",
                inkwell::OptimizationLevel::Default,
                RelocMode::PIC,
                CodeModel::Default,
            )
            .unwrap();

        Ok(target_machine)
    }

    pub fn optimize(&self, target_machine: &TargetMachine) -> Result<(), Error> {
        let passes: &[&str] = &[
            "instcombine",
            "reassociate",
            "gvn",
            "simplifycfg",
            "mem2reg",
            "dse",
            "loop-simplify",
            "indvars",
            "loop-unroll",
            "jump-threading",
            "sccp",
            "dce",
            "sink",
            "tailcallelim",
        ];

        self.module
            .run_passes(
                passes.join(",").as_str(),
                target_machine,
                PassBuilderOptions::create(),
            )
            .map_err(|err| Error::Other(err.to_string()))?;

        Ok(())
    }

    pub fn compile(&self, target_machine: &TargetMachine, dest_path: &Path) -> Result<(), Error> {
        target_machine
            .write_to_file(&self.module, FileType::Object, dest_path)
            .map_err(|err| Error::Other(err.to_string()))?;

        Ok(())
    }

    pub fn emit_program(&mut self, ast: &[Ast]) -> Result<(), Error> {
        let mut function_decls = vec![];
        let mut statements = vec![];

        ast.iter().for_each(|f| match f {
            Ast::FunctionDeclaration(_, _, _) => function_decls.push(f),
            _ => statements.push(f),
        });

        for astnode in function_decls {
            self.emit_function_declaration(astnode)?;
        }

        let main_function_type = self.context.f32_type().fn_type(&[], false);
        let main_function = self.module.add_function("main", main_function_type, None);

        let bb = self.context.append_basic_block(main_function, "entry");
        self.builder.position_at_end(bb);

        self.current_function = Some(main_function);

        for astnode in statements {
            self.emit_statement(astnode);
        }

        self.builder
            .build_return(Some(&self.context.f32_type().const_float(0.0)))
            .unwrap();

        Ok(())
    }

    fn create_printf(
        context: &'ctx Context,
        module: &Module<'ctx>,
    ) -> (FunctionValue<'ctx>, GlobalValue<'ctx>) {
        let printf_format = "%lld\n";
        let printf_format_type = context
            .i8_type()
            .array_type((printf_format.len() + 1) as u32);
        let printf_format_global = module.add_global(printf_format_type, None, "write_format");

        printf_format_global.set_initializer(&context.const_string(printf_format.as_bytes(), true));

        let printf_args = [context.ptr_type(AddressSpace::default()).into()];

        let printf_type = context.f32_type().fn_type(&printf_args, true);
        let printf_fn = module.add_function("printf", printf_type, None);

        (printf_fn, printf_format_global)
    }

    pub fn emit_write(&mut self, value: &Expression) {
        let value = self.emit_expression(value).unwrap();

        let args: &[BasicMetadataValueEnum<'ctx>] =
            &[self.printf.1.as_pointer_value().into(), value.into()];

        self.builder
            .build_call(self.printf.0, args, "write_call")
            .unwrap();
    }

    fn create_scanf(
        context: &'ctx Context,
        module: &Module<'ctx>,
    ) -> (FunctionValue<'ctx>, GlobalValue<'ctx>) {
        let scanf_format = "%lld";
        let scanf_format_type = context
            .i8_type()
            .array_type((scanf_format.len() + 1) as u32);
        let scanf_format_global = module.add_global(scanf_format_type, None, "read_format");

        scanf_format_global.set_initializer(&context.const_string(scanf_format.as_bytes(), true));

        let scanf_args = [context.ptr_type(AddressSpace::default()).into()];

        let scanf_type = context.f32_type().fn_type(&scanf_args, true);
        let scanf_fn = module.add_function("scanf", scanf_type, None);

        (scanf_fn, scanf_format_global)
    }

    pub fn declare_functions(&mut self, functions: &Vec<Ast>) {
        for function in functions {
            if let Ast::FunctionDeclaration(name, args, _) = function {
                let function_params = args
                    .iter()
                    .map(|_| self.context.f32_type().into())
                    .collect::<Vec<BasicMetadataTypeEnum<'ctx>>>();

                let function_type = self.context.f32_type().fn_type(&function_params, false);

                let llvm_function = self.module.add_function(name, function_type, None);

                self.functions
                    .insert(name.clone(), Function { ptr: llvm_function });
            }
        }
    }

    pub fn emit_function_declaration(&mut self, astnode: &Ast) -> Result<(), Error> {
        if let Ast::FunctionDeclaration(name, args, expr) = astnode {
            let function = self.functions.get(name).unwrap().ptr;

            let entry_bb = self.context.append_basic_block(function, "entry");

            self.current_function = Some(function);

            self.builder.position_at_end(entry_bb);

            for (idx, arg) in args.iter().enumerate() {
                let value = function.get_nth_param(idx as u32).unwrap();

                let alloca_ptr = self.builder.build_alloca(self.context.f32_type(), arg)?;

                self.builder.build_store(alloca_ptr, value)?;
                self.variables
                    .insert(arg.clone(), Variable { ptr: alloca_ptr });
            }

            self.emit_return(expr);
        }

        Ok(())
    }

    pub fn emit_statement(&mut self, astnode: &Ast) {
        if let Ast::Assignment(ident, expr) = astnode {
            if !self.variables.contains_key(ident) {
                self.emit_declaration(ident, expr)
            } else {
                self.emit_assignment(ident, expr)
            }
        } else if let Ast::FunctionCall(_, _) = astnode {
            self.emit_function_call(astnode).unwrap();
        }
    }

    pub fn emit_declaration(&mut self, name: &String, expr: &Expression) {
        let float_type = self.context.f32_type();

        let alloca_ptr = self.builder.build_alloca(float_type, name).unwrap();

        self.variables
            .insert(name.to_owned(), Variable { ptr: alloca_ptr });

        let init_val = self.emit_expression(expr).unwrap();

        self.builder.build_store(alloca_ptr, init_val).unwrap();
    }

    pub fn emit_assignment(&mut self, name: &String, expr: &Expression) {
        let variable = self.variables.get(name).unwrap().clone();

        let val = self.emit_expression(expr).unwrap();

        self.builder.build_store(variable.ptr, val).unwrap();
    }

    pub fn emit_expression(&mut self, expr: &ast::Expression) -> Result<FloatValue<'ctx>, Error> {
        Ok(match expr {
            Expression::Number(value) => self.emit_float(*value),
            Expression::Identifier(var) => self.emit_variable(var)?,
            Expression::FunctionCall(name, args) => {
                self.emit_function_call(&Ast::FunctionCall(name.to_owned(), args.to_owned()))?
            }
            Expression::Binary(_, _, _) => self.emit_binary_op(expr)?,
            _ => unreachable!(),
        })
    }

    pub fn emit_return(&mut self, retval: &ast::Expression) {
        let retval = self.emit_expression(retval).unwrap();
        self.builder.build_return(Some(&retval)).unwrap();
    }

    pub fn emit_float(&mut self, value: f32) -> FloatValue<'ctx> {
        self.context.f32_type().const_float(value as f64)
    }

    pub fn emit_variable(&mut self, name: &str) -> Result<FloatValue<'ctx>, Error> {
        let variable = self.variables.get(name).unwrap();

        let value = self
            .builder
            .build_load(self.context.f32_type(), variable.ptr, name)?;

        Ok(value.into_float_value())
    }

    pub fn emit_function_call(&mut self, astnode: &Ast) -> Result<FloatValue<'ctx>, Error> {
        match astnode {
            Ast::FunctionCall(name, args) => {
                let function = self.functions.get(name).cloned().unwrap();

                let exprs: Vec<BasicMetadataValueEnum<'ctx>> = args
                    .iter()
                    .map(|arg| self.emit_expression(arg).map(Into::into))
                    .collect::<Result<_, _>>()?;

                let retval =
                    self.builder
                        .build_call(function.ptr, &exprs, &format!("{}_call", &name))?;

                Ok(retval.try_as_basic_value().unwrap_left().into_float_value())
            }
            _ => unreachable!(),
        }
    }

    pub fn emit_binary_op(&mut self, binary_op: &Expression) -> Result<FloatValue<'ctx>, Error> {
        if let Expression::Binary(lhs, op, rhs) = binary_op {
            let left = self.emit_expression(lhs)?;
            let right = self.emit_expression(rhs)?;

            let result = match op {
                Token::Add => self.builder.build_float_add(left, right, "add")?,
                Token::Sub => self.builder.build_float_sub(left, right, "sub")?,
                Token::Mul => self.builder.build_float_mul(left, right, "mul")?,
                Token::Div => self.builder.build_float_div(left, right, "div")?,
                Token::Mod => self.builder.build_float_rem(left, right, "mod")?,
                Token::Eq => self
                    .builder
                    .build_float_compare(FloatPredicate::OEQ, left, right, "eq")?
                    .as_basic_value_enum()
                    .into_float_value(),
                Token::NEq => self
                    .builder
                    .build_float_compare(FloatPredicate::ONE, left, right, "neq")?
                    .as_basic_value_enum()
                    .into_float_value(),
                Token::Lt => self
                    .builder
                    .build_float_compare(FloatPredicate::OLT, left, right, "lt")?
                    .as_basic_value_enum()
                    .into_float_value(),
                Token::LtEq => self
                    .builder
                    .build_float_compare(FloatPredicate::OLE, left, right, "lte")?
                    .as_basic_value_enum()
                    .into_float_value(),
                Token::Gt => self
                    .builder
                    .build_float_compare(FloatPredicate::OGT, left, right, "gt")?
                    .as_basic_value_enum()
                    .into_float_value(),
                Token::GtEq => self
                    .builder
                    .build_float_compare(FloatPredicate::OGE, left, right, "gte")?
                    .as_basic_value_enum()
                    .into_float_value(),
                _ => unreachable!(),
            };

            Ok(result)
        } else {
            unimplemented!()
        }
    }
}

#[derive(Debug, Clone)]
struct Function<'ctx> {
    pub ptr: FunctionValue<'ctx>,
}

#[derive(Debug, Clone)]
pub struct Variable<'ctx> {
    ptr: PointerValue<'ctx>,
}
