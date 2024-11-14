use std::{collections::HashMap, path::Path};

use inkwell::{
	builder::Builder,
	context::Context,
	module::Module,
	passes::PassBuilderOptions,
	targets::{CodeModel, FileType, InitializationConfig, RelocMode, Target, TargetMachine},
	types::{BasicMetadataTypeEnum, BasicTypeEnum},
	values::{
		ArrayValue, BasicMetadataValueEnum, BasicValue, BasicValueEnum, FunctionValue, GlobalValue,
		PointerValue,
	},
	AddressSpace, FloatPredicate, IntPredicate,
};

use crate::{
	ast::{self, Ast, Expression},
	token::Token,
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error("Builder error: {0}")]
	Builder(#[from] inkwell::builder::BuilderError),

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
	standard_functions: Vec<&'ctx str>,

	printf: (FunctionValue<'ctx>, GlobalValue<'ctx>),
	scanf: (FunctionValue<'ctx>, GlobalValue<'ctx>),
}

impl<'ctx> Compiler<'ctx> {
	pub fn new(context: &'ctx Context, module_name: &str) -> Self {
		let module = context.create_module(module_name);
		let builder = context.create_builder();
		let printf = Compiler::create_printf(context, &module);
		let scanf = Compiler::create_scanf(context, &module);
		let standard_functions = vec![
			"print", "read", "round", "ceil", "floor", "log", "sin", "cos", "tan", "sqrt",
		];

		let f64_type = context.f64_type();

		let di_intrinsic_type = f64_type.fn_type(&[f64_type.into(), f64_type.into()], false);
		let intrinsic_type = f64_type.fn_type(&[f64_type.into()], false);

		module.add_function("llvm.pow.f64", di_intrinsic_type, None);
		module.add_function("llvm.fabs.f64", intrinsic_type, None);
		module.add_function("llvm.round.f64", intrinsic_type, None);
		module.add_function("llvm.ceil.f64", intrinsic_type, None);
		module.add_function("llvm.floor.f64", intrinsic_type, None);
		module.add_function("llvm.log.f64", intrinsic_type, None);
		module.add_function("llvm.sin.f64", intrinsic_type, None);
		module.add_function("llvm.cos.f64", intrinsic_type, None);
		module.add_function("llvm.tan.f64", intrinsic_type, None);
		module.add_function("llvm.sqrt.f64", intrinsic_type, None);

		Self {
			context,
			module,
			builder,
			printf,
			scanf,
			current_function: None,
			standard_functions,
			variables: HashMap::default(),
			functions: HashMap::default(),
		}
	}

	pub fn write_ir(&self, file: String) {
		let _ = self.module.print_to_file(file);
	}

	pub fn verify(&self) -> Result<(), Error> {
		self
			.module
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
				inkwell::OptimizationLevel::Aggressive,
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

		self
			.module
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

	pub fn emit_main(&mut self, ast: &[Ast]) -> Result<(), Error> {
		let mut statements = vec![];

		for node in ast {
			match node {
				Ast::FunctionDeclaration(..) => self.emit_function_declaration(node)?,
				_ => statements.push(node),
			}
		}

		let main_function_type = self.context.i32_type().fn_type(&[], false);
		let main_function = self.module.add_function("main", main_function_type, None);

		let basic_block = self.context.append_basic_block(main_function, "entry");
		self.builder.position_at_end(basic_block);

		self.current_function = Some(main_function);

		for astnode in statements {
			self.emit_statement(astnode)?;
		}

		self
			.builder
			.build_return(Some(&self.context.i32_type().const_int(0, false)))?;

		Ok(())
	}

	fn create_printf(
		context: &'ctx Context,
		module: &Module<'ctx>,
	) -> (FunctionValue<'ctx>, GlobalValue<'ctx>) {
		let printf_format = "%lf\n";
		let printf_format_type = context
			.i8_type()
			.array_type((printf_format.len() + 1) as u32);
		let printf_format_global = module.add_global(printf_format_type, None, "print_format");

		printf_format_global.set_initializer(&context.const_string(printf_format.as_bytes(), true));

		let printf_args = [context.ptr_type(AddressSpace::default()).into()];

		let printf_type = context.i32_type().fn_type(&printf_args, true);
		let printf_fn = module.add_function("printf", printf_type, None);

		(printf_fn, printf_format_global)
	}

	pub fn emit_print(&mut self, value: &Expression) {
		let value = self.emit_expression(value).unwrap();

		if value.get_type().is_array_type() {
			
			let array = value.into_array_value();

			let variable = self
				.variables
				.get(array.get_name().to_str().unwrap())
				.unwrap();

			let array_ptr = variable.ptr;
			let array_size = variable.r#type.into_array_type().len();

			let i64_type = self.context.i64_type();
			let f64_type = self.context.f64_type();

			let counter_ptr = self.builder.build_alloca(i64_type, "counter").unwrap();

			let main_block = self
				.builder
				.build_store(counter_ptr, i64_type.const_int(0, false))
				.unwrap().get_parent().unwrap();

			let loop_block = self
				.context
				.append_basic_block(self.current_function.unwrap(), "begin_print_loop");

			let end_block = self
				.context
				.append_basic_block(self.current_function.unwrap(), "end_print_loop");

			self.builder.build_unconditional_branch(loop_block).unwrap();
			self.builder.position_at_end(loop_block);

			let counter = self
				.builder
				.build_load(i64_type, counter_ptr, "counter")
				.unwrap();

			let indices = [counter.into_int_value()];

			let element = unsafe {
				self
					.builder
					.build_gep(f64_type, array_ptr, &indices, "element")
			}
			.unwrap();

			let value = self.builder.build_load(f64_type, element, "value").unwrap();

			let args = &[self.printf.1.as_pointer_value().into(), value.into()];

			self
				.builder
				.build_call(self.printf.0, args, "print_call")
				.unwrap();

			let new_counter = self
				.builder
				.build_int_add(
					counter.into_int_value(),
					i64_type.const_int(1, false),
					"new_counter",
				)
				.unwrap();

			self.builder.build_store(counter_ptr, new_counter).unwrap();

			let comparison = self
				.builder
				.build_int_compare(
					IntPredicate::EQ,
					new_counter,
					i64_type.const_int(array_size.into(), false),
					"eq",
				)
				.unwrap();

			self
				.builder
				.build_conditional_branch(comparison, end_block, loop_block)
				.unwrap();

			self.builder.position_at_end(end_block);
			self
				.builder
				.build_return(Some(&self.context.i32_type().const_int(0, false)))
				.unwrap();

			self.builder.position_at_end(main_block)
		} else {
			let args = &[self.printf.1.as_pointer_value().into(), value.into()];

			self
				.builder
				.build_call(self.printf.0, args, "print_call")
				.unwrap();
		}
	}

	fn create_scanf(
		context: &'ctx Context,
		module: &Module<'ctx>,
	) -> (FunctionValue<'ctx>, GlobalValue<'ctx>) {
		let scanf_format = "%lf";
		let scanf_format_type = context
			.i8_type()
			.array_type((scanf_format.len() + 1) as u32);
		let scanf_format_global = module.add_global(scanf_format_type, None, "read_format");

		scanf_format_global.set_initializer(&context.const_string(scanf_format.as_bytes(), true));

		let scanf_args = [context.ptr_type(AddressSpace::default()).into()];

		let scanf_type = context.f64_type().fn_type(&scanf_args, true);
		let scanf_fn = module.add_function("scanf", scanf_type, None);

		(scanf_fn, scanf_format_global)
	}

	pub fn emit_read(&mut self) -> Result<BasicValueEnum<'ctx>, Error> {
		let result_value = self
			.builder
			.build_alloca(self.context.i64_type(), "read_result")?;

		let args: &[BasicMetadataValueEnum<'ctx>] =
			&[self.scanf.1.as_pointer_value().into(), result_value.into()];

		self.builder.build_call(self.scanf.0, args, "read_call")?;

		Ok(
			self
				.builder
				.build_load(self.context.i64_type(), result_value, "read_result")?,
		)
	}

	pub fn declare_globals(&mut self, variables: &Vec<Ast>) {
		let f64_type = self.context.f64_type();
		for variable in variables {
			if let Ast::Assignment(name, value) = variable {
				if self.variables.contains_key(name) {
					continue;
				}

				let alloca_ptr;

				if let Expression::SizedSet(set) = value {
					let array_type = f64_type.array_type(set.len().try_into().unwrap());
					alloca_ptr = self.module.add_global(array_type, None, name);

					self.variables.insert(
						name.to_owned(),
						Variable {
							ptr: alloca_ptr.as_pointer_value(),
							r#type: BasicTypeEnum::ArrayType(array_type),
						},
					);

					let mut array_values = vec![];
					set.iter().for_each(|f| {
						array_values.push(self.emit_expression(f).unwrap().into_float_value());
					});

					let ret = unsafe { ArrayValue::new_const_array(&f64_type, &array_values) };

					alloca_ptr.set_initializer(&ret);
				} else {
					alloca_ptr = self.module.add_global(f64_type, None, name);

					self.variables.insert(
						name.to_owned(),
						Variable {
							ptr: alloca_ptr.as_pointer_value(),
							r#type: BasicTypeEnum::FloatType(f64_type),
						},
					);

					alloca_ptr.set_initializer(
						&self
							.context
							.f64_type()
							.const_float(0.0)
							.as_basic_value_enum(),
					)
				}
			}
		}
	}

	pub fn declare_functions(&mut self, functions: &Vec<Ast>) {
		for function in functions {
			if let Ast::FunctionDeclaration(name, args, _) = function {
				let function_params = args
					.iter()
					.map(|_| self.context.f64_type().into())
					.collect::<Vec<BasicMetadataTypeEnum<'ctx>>>();

				let function_type = self.context.f64_type().fn_type(&function_params, false);

				let llvm_function = self.module.add_function(name, function_type, None);

				self
					.functions
					.insert(name.clone(), Function { ptr: llvm_function });
			}
		}
	}

	pub fn emit_function_declaration(&mut self, astnode: &Ast) -> Result<(), Error> {
		if let Ast::FunctionDeclaration(name, args, expr) = astnode {
			let function = self.functions.get(name).unwrap().ptr;

			let entry_basic_block = self.context.append_basic_block(function, "entry");

			self.current_function = Some(function);

			self.builder.position_at_end(entry_basic_block);

			for (idx, arg) in args.iter().enumerate() {
				let value = function.get_nth_param(idx as u32).unwrap();
				let f64_type = self.context.f64_type();

				let alloca_ptr = self.builder.build_alloca(f64_type, arg)?;

				self.builder.build_store(alloca_ptr, value)?;
				self.variables.insert(
					arg.clone(),
					Variable {
						ptr: alloca_ptr,
						r#type: BasicTypeEnum::FloatType(f64_type),
					},
				);
			}

			self.emit_return(expr)?;
		}

		Ok(())
	}

	pub fn emit_statement(&mut self, astnode: &Ast) -> Result<(), Error> {
		if let Ast::Assignment(ident, expr) = astnode {
			self.emit_assignment(ident, expr)?;
		} else if let Ast::FunctionCall(..) = astnode {
			self.emit_function_call(astnode)?;
		}

		Ok(())
	}

	pub fn emit_assignment(&mut self, name: &String, expr: &Expression) -> Result<(), Error> {
		let variable = self.variables.get(name).unwrap().clone();

		let val = self.emit_expression(expr)?;

		self.builder.build_store(variable.ptr, val)?;

		Ok(())
	}

	pub fn emit_expression(&mut self, expr: &ast::Expression) -> Result<BasicValueEnum<'ctx>, Error> {
		Ok(match expr {
			Expression::Abs(expr) => {
				let arg = self.emit_expression(expr)?.into();
				self
					.builder
					.build_direct_call(
						self.module.get_function("llvm.fabs.f64").unwrap(),
						&[arg],
						"ret",
					)?
					.try_as_basic_value()
					.left()
					.unwrap()
			}
			Expression::Number(value) => self.emit_float(*value),
			Expression::Identifier(var) => self.emit_variable(var)?,
			Expression::FunctionCall(name, args) => {
				self.emit_function_call(&Ast::FunctionCall(name.to_owned(), args.to_owned()))?
			}
			Expression::Binary(..) => self.emit_binary_op(expr)?,
			Expression::Branched(..) => self.emit_branched_expr(expr)?,
			Expression::SizedSet(..) => self.emit_set(expr)?,
		})
	}

	pub fn emit_return(&mut self, retval: &ast::Expression) -> Result<(), Error> {
		let retval = self.emit_expression(retval)?;
		self.builder.build_return(Some(&retval))?;

		Ok(())
	}

	pub fn emit_float(&mut self, value: f64) -> BasicValueEnum<'ctx> {
		self
			.context
			.f64_type()
			.const_float(value)
			.as_basic_value_enum()
	}

	pub fn emit_variable(&mut self, name: &str) -> Result<BasicValueEnum<'ctx>, Error> {
		let variable = self.variables.get(name).unwrap();

		let value = self
			.builder
			.build_load(variable.r#type, variable.ptr, name)?;

		Ok(value)
	}

	pub fn emit_function_call(&mut self, astnode: &Ast) -> Result<BasicValueEnum<'ctx>, Error> {
		match astnode {
			Ast::FunctionCall(name, args) => {
				if self.standard_functions.contains(&name.as_str()) {
					return self.emit_standard_function_call(name, args);
				}

				let exprs: Vec<BasicMetadataValueEnum<'ctx>> = args
					.iter()
					.map(|arg| self.emit_expression(arg).map(Into::into))
					.collect::<Result<_, _>>()?;

				let function = self.functions.get(name).cloned().unwrap();

				let retval = self
					.builder
					.build_call(function.ptr, &exprs, &format!("{}_call", &name))?;

				Ok(retval.try_as_basic_value().unwrap_left())
			}
			_ => unreachable!(),
		}
	}

	pub fn emit_standard_function_call(
		&mut self,
		name: &str,
		args: &[Expression],
	) -> Result<BasicValueEnum<'ctx>, Error> {
		let default_return_value = self
			.context
			.f64_type()
			.const_float(0.0)
			.as_basic_value_enum();

		match name {
			"print" => {
				self.emit_print(&args[0]);
				Ok(default_return_value)
			}
			"read" => self.emit_read(),
			"round" | "ceil" | "floor" | "log" | "sin" | "cos" | "tan" | "sqrt" => {
				let intrinsic = self
					.module
					.get_function(&format!("llvm.{name}.f64"))
					.unwrap();
				let args: Vec<BasicMetadataValueEnum<'ctx>> = args
					.iter()
					.map(|arg| self.emit_expression(arg).map(Into::into))
					.collect::<Result<_, _>>()?;

				let res = self
					.builder
					.build_direct_call(intrinsic, &args, "ret")?
					.try_as_basic_value()
					.left()
					.unwrap();

				Ok(res)
			}

			_ => unimplemented!(),
		}
	}

	pub fn emit_set(&mut self, set: &Expression) -> Result<BasicValueEnum<'ctx>, Error> {
		if let Expression::SizedSet(set) = set {
			let f64_type = self.context.f64_type();

			let mut array_values = vec![];
			set.iter().for_each(|f| {
				array_values.push(self.emit_expression(f).unwrap().into_float_value());
			});

			let ret = unsafe { ArrayValue::new_const_array(&f64_type, &array_values) };

			Ok(ret.as_basic_value_enum())
		} else {
			unreachable!()
		}
	}

	pub fn emit_binary_op(&mut self, binary_op: &Expression) -> Result<BasicValueEnum<'ctx>, Error> {
		if let Expression::Binary(lhs, op, rhs) = binary_op {
			let left = self.emit_expression(lhs)?.into_float_value();
			let right = self.emit_expression(rhs)?.into_float_value();

			let result = match op {
				Token::Add => self
					.builder
					.build_float_add(left, right, "add")?
					.as_basic_value_enum(),
				Token::Sub => self
					.builder
					.build_float_sub(left, right, "sub")?
					.as_basic_value_enum(),
				Token::Mul => self
					.builder
					.build_float_mul(left, right, "mul")?
					.as_basic_value_enum(),
				Token::Div => self
					.builder
					.build_float_div(left, right, "div")?
					.as_basic_value_enum(),
				Token::Pow => self
					.builder
					.build_direct_call(
						self.module.get_function("llvm.pow.f64").unwrap(),
						&[left.into(), right.into()],
						"ret",
					)?
					.try_as_basic_value()
					.left()
					.unwrap(),
				Token::Rem => self
					.builder
					.build_float_rem(left, right, "mod")?
					.as_basic_value_enum(),
				Token::IsEq => self
					.builder
					.build_float_compare(FloatPredicate::OEQ, left, right, "eq")?
					.as_basic_value_enum(),
				Token::NEq => self
					.builder
					.build_float_compare(FloatPredicate::ONE, left, right, "neq")?
					.as_basic_value_enum(),
				Token::Lt => self
					.builder
					.build_float_compare(FloatPredicate::OLT, left, right, "lt")?
					.as_basic_value_enum(),
				Token::LtEq => self
					.builder
					.build_float_compare(FloatPredicate::OLE, left, right, "lte")?
					.as_basic_value_enum(),
				Token::Gt => self
					.builder
					.build_float_compare(FloatPredicate::OGT, left, right, "gt")?
					.as_basic_value_enum(),
				Token::GtEq => self
					.builder
					.build_float_compare(FloatPredicate::OGE, left, right, "gte")?
					.as_basic_value_enum(),
				_ => unreachable!("{op:?}"),
			};

			Ok(result)
		} else {
			unreachable!()
		}
	}

	pub fn emit_branched_expr(
		&mut self,
		branched_expr: &Expression,
	) -> Result<BasicValueEnum<'ctx>, Error> {
		if let Expression::Branched(condition, expr1, expr2) = branched_expr {
			let btrue = self
				.context
				.append_basic_block(self.current_function.unwrap(), "btrue");
			let bfalse = self
				.context
				.append_basic_block(self.current_function.unwrap(), "bfalse");
			let end = self
				.context
				.append_basic_block(self.current_function.unwrap(), "end");

			let condition = self.emit_expression(condition)?;
			let float_type = self.context.f64_type();

			let alloca_ptr = self.builder.build_alloca(float_type, "retvalue")?;

			self
				.builder
				.build_conditional_branch(condition.into_int_value(), btrue, bfalse)?;

			self.builder.position_at_end(btrue);
			let value = self.emit_expression(expr1)?;
			self.builder.build_store(alloca_ptr, value)?;
			self.builder.build_unconditional_branch(end)?;

			self.builder.position_at_end(bfalse);
			let value = self.emit_expression(expr2)?;
			self.builder.build_store(alloca_ptr, value)?;
			self.builder.build_unconditional_branch(end)?;

			self.builder.position_at_end(end);

			let retvalue = self.builder.build_load(float_type, alloca_ptr, "ret")?;

			Ok(retvalue)
		} else {
			unreachable!()
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
	r#type: BasicTypeEnum<'ctx>,
}
