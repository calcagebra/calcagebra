use std::collections::HashMap;

use codegen::{ir, verify_function};
use cranelift::{
	jit::{JITBuilder, JITModule},
	module::{default_libcall_names, DataDescription, Linkage, Module},
	native::builder,
	prelude::*,
};

use crate::ast::{AstNode, Expression};

/// The basic JIT class.
pub struct JIT {
	/// The function builder context, which is reused across multiple
	/// FunctionBuilder instances.
	builder_context: FunctionBuilderContext,

	/// The main Cranelift context, which holds the state for codegen. Cranelift
	/// separates this from `Module` to allow for parallel compilation, with a
	/// context per thread, though this isn't in the simple demo here.
	ctx: codegen::Context,

	/// The data description, which is to data objects what `ctx` is to functions.
	data_description: DataDescription,

	/// The module, with the jit backend, which manages the JIT'd
	/// functions.
	module: JITModule,
}

// Prints a value used by the compiled code. Our JIT exposes this
// function to compiled code with the name "print".
unsafe extern fn print_internal(value: f64) -> f64 {
	println!("{}", value);
	0.0
}

impl Default for JIT {
	fn default() -> Self {
		let mut flag_builder = settings::builder();
		flag_builder.set("use_colocated_libcalls", "false").unwrap();
		flag_builder.set("is_pic", "false").unwrap();
		let isa_builder = builder().unwrap_or_else(|msg| {
			panic!("host machine is not supported: {}", msg);
		});
		let isa = isa_builder
			.finish(settings::Flags::new(flag_builder))
			.unwrap();
		let mut builder = JITBuilder::with_isa(isa, default_libcall_names());

		let print_addr = print_internal as *const u8;
		builder.symbol("print", print_addr);

		let module = JITModule::new(builder);
		Self {
			builder_context: FunctionBuilderContext::new(),
			ctx: module.make_context(),
			data_description: DataDescription::new(),
			module,
		}
	}
}

impl JIT {
	pub fn execute(&mut self, ast: Vec<AstNode>) -> Result<*const u8, String> {
		let ty = types::F64;

		let mut builder = FunctionBuilder::new(&mut self.ctx.func, &mut self.builder_context);
		let entry_block = builder.create_block();

		builder.append_block_params_for_function_params(entry_block);
		builder.switch_to_block(entry_block);
		builder.seal_block(entry_block);

		let mut translator = Translator {
			ty,
			builder,
			variables: HashMap::new(),
			module: &mut self.module,
		};

		for node in ast {
			match node {
				AstNode::Assignment(name, expression) => {
					let idx = translator.variables.len();
					let var = Variable::new(idx);
					translator.variables.insert(name.to_string(), var);
					translator.builder.declare_var(var, ty);

					let val = translator.translate_expr(expression);

					translator.builder.def_var(var, val);
				}
				AstNode::FunctionCall(ident, args) => {
					let standard_functions = ["print"].map(String::from);

					if standard_functions.contains(&ident) {
						let mut sig = translator.module.make_signature();

						for _arg in &args {
							sig.params.push(AbiParam::new(translator.ty));
						}

						sig.returns.push(AbiParam::new(translator.ty));

						let callee = translator
							.module
							.declare_function(&ident, Linkage::Local, &sig)
							.expect("problem declaring function");

						let local_callee: ir::FuncRef = translator
							.module
							.declare_func_in_func(callee, translator.builder.func);

						let mut arg_values = Vec::new();
						for arg in args {
							arg_values.push(translator.translate_expr(arg))
						}
						let call = translator.builder.ins().call(local_callee, &arg_values);
						assert!(!translator.builder.inst_results(call).is_empty());
					}
				}
				AstNode::FunctionDeclaration(_, vec, expression) => todo!(),
			}
		}

		translator.builder.ins().return_(&[]);

		if let Err(errors) = verify_function(&self.ctx.func, self.module.isa()) {
			eprintln!("Verifier errors: {}", errors);
		}

		println!("{}", self.ctx.func.display());

		let id = self
			.module
			.declare_function("main", Linkage::Export, &self.ctx.func.signature)
			.map_err(|e| e.to_string())?;

		self
			.module
			.define_function(id, &mut self.ctx)
			.map_err(|e| e.to_string())?;

		self.module.clear_context(&mut self.ctx);

		self.module.finalize_definitions().unwrap();

		let code = self.module.get_finalized_function(id);

		Ok(code)
	}
}

struct Translator<'a> {
	ty: types::Type,
	builder: FunctionBuilder<'a>,
	variables: HashMap<String, Variable>,
	module: &'a mut JITModule,
}

impl Translator<'_> {
	fn translate(&mut self, ast: Vec<AstNode>) -> Result<(), String> {
		Ok(())
	}

	pub fn translate_expr(&mut self, expr: Expression) -> Value {
		match expr {
			Expression::Abs(expression) => todo!(),
			Expression::Binary(expression, token, expression1) => todo!(),
			Expression::Branched(expression, expression1, expression2) => todo!(),
			Expression::Identifier(ident) => {
				let variable = self.variables.get(&ident).expect("variable not defined");
				self.builder.use_var(*variable)
			}
			Expression::Number(n) => self.builder.ins().f64const(n),
			Expression::FunctionCall(_, vec) => todo!(),
			Expression::SizedSet(vec) => todo!(),
		}
	}
}
