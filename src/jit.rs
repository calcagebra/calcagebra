use std::collections::HashMap;

use codegen::verify_function;
use cranelift::{
	jit::{JITBuilder, JITModule},
	module::{default_libcall_names, FuncId, Linkage, Module},
	native::builder,
	prelude::*,
};

use crate::{
	ast::{AstNode, Expression},
	standardlibrary::*,
	token::Token,
};

/// The basic JIT class.
pub struct Jit {
	/// The function builder context, which is reused across multiple
	/// FunctionBuilder instances.
	builder_context: FunctionBuilderContext,

	/// The main Cranelift context, which holds the state for codegen. Cranelift
	/// separates this from `Module` to allow for parallel compilation, with a
	/// context per thread, though this isn't in the simple demo here.
	ctx: codegen::Context,

	/// The module, with the jit backend, which manages the JIT'd
	/// functions.
	module: JITModule,

	variables: HashMap<String, (Variable, Option<Expression>)>,
	functions: HashMap<String, FuncId>,

	counter: isize,
}

impl Default for Jit {
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

		// IO
		builder.symbol("print", print as *const u8);
		builder.symbol("read", read as *const u8);

		// MATH
		builder.symbol("round", round as *const u8);
		builder.symbol("ceil", ceil as *const u8);
		builder.symbol("floor", floor as *const u8);
		builder.symbol("ln", ln as *const u8);
		builder.symbol("log10", log10 as *const u8);
		builder.symbol("log", log as *const u8);
		builder.symbol("log10", log10 as *const u8);
		builder.symbol("sin", sin as *const u8);
		builder.symbol("cos", cos as *const u8);
		builder.symbol("tan", tan as *const u8);
		builder.symbol("sqrt", sqrt as *const u8);
		builder.symbol("cbrt", cbrt as *const u8);
		builder.symbol("nrt", nrt as *const u8);
		builder.symbol("pow", pow as *const u8);
		builder.symbol("graph", graph as *const u8);

		let module = JITModule::new(builder);
		Self {
			builder_context: FunctionBuilderContext::new(),
			ctx: module.make_context(),
			variables: HashMap::new(),
			functions: HashMap::new(),
			module,
			counter: 0,
		}
	}
}

impl Jit {
	pub fn execute(&mut self, ast: Vec<AstNode>) -> Result<*const u8, String> {
		let ty = types::F64;

		let mut filtered_ast = vec![];
		let mut functions = self.functions.clone();

		for node in ast {
			match node {
				AstNode::FunctionDeclaration(name, params, expr) => {
					for _p in &params {
						self.ctx.func.signature.params.push(AbiParam::new(ty));
					}

					self.ctx.func.signature.returns.push(AbiParam::new(ty));

					let mut builder = FunctionBuilder::new(&mut self.ctx.func, &mut self.builder_context);
					let entry_block = builder.create_block();

					builder.append_block_params_for_function_params(entry_block);
					builder.switch_to_block(entry_block);
					builder.seal_block(entry_block);

					let mut variables = HashMap::new();
					let mut index = 0;

					for (i, name) in params.iter().enumerate() {
						let val = builder.block_params(entry_block)[i];
						let var = Variable::new(index);
						if !variables.contains_key(name) {
							variables.insert(name.into(), (var, None));
							builder.declare_var(var, ty);
							index += 1;
						}
						builder.def_var(var, val);
					}

					let mut trans = Translator {
						ty,
						builder,
						variables,
						functions: HashMap::new(),
						module: &mut self.module,
					};

					let ret_value = trans.translate_expr(&expr);
					trans.builder.ins().return_(&[ret_value]);

					trans.builder.finalize();

					if let Err(errors) = verify_function(&self.ctx.func, self.module.isa()) {
						eprintln!("Verifier errors: {}", errors);
					}

					let id = self
						.module
						.declare_function(&name, Linkage::Export, &self.ctx.func.signature)
						.map_err(|e| e.to_string())?;

					functions.insert(name, id);

					self
						.module
						.define_function(id, &mut self.ctx)
						.map_err(|e| e.to_string())?;

					if let Err(errors) = verify_function(&self.ctx.func, self.module.isa()) {
						eprintln!("Verifier errors: {}", errors);
					}

					self.module.clear_context(&mut self.ctx);

					self.module.finalize_definitions().unwrap();
				}
				_ => filtered_ast.push(node),
			}
		}

		let mut builder = FunctionBuilder::new(&mut self.ctx.func, &mut self.builder_context);
		let entry_block = builder.create_block();

		builder.append_block_params_for_function_params(entry_block);
		builder.switch_to_block(entry_block);
		builder.seal_block(entry_block);

		let mut trans = Translator {
			ty,
			builder,
			variables: HashMap::new(),
			functions,
			module: &mut self.module,
		};

		for (name, (variable, expression)) in &self.variables {
			let val = trans.translate_expr(expression.as_ref().unwrap());

			trans
				.variables
				.insert(name.to_string(), (*variable, expression.clone()));
			trans.builder.declare_var(*variable, ty);

			trans.builder.def_var(*variable, val);
		}

		for node in filtered_ast {
			match node {
				AstNode::Assignment(name, expression) => {
					let idx = trans.variables.len();
					let var = Variable::new(idx);

					let val = trans.translate_expr(&expression);

					trans
						.variables
						.insert(name.to_string(), (var, Some(expression)));
					trans.builder.declare_var(var, ty);

					trans.builder.def_var(var, val);
				}
				AstNode::FunctionCall(ident, args) => {
					let mut sig = trans.module.make_signature();

					for _arg in &args {
						sig.params.push(AbiParam::new(trans.ty));
					}

					sig.returns.push(AbiParam::new(trans.ty));

					let callee = trans
						.module
						.declare_function(&ident, Linkage::Local, &sig)
						.expect("problem declaring function");

					let local_callee = trans
						.module
						.declare_func_in_func(callee, trans.builder.func);

					let mut arg_values = Vec::new();
					for arg in args {
						arg_values.push(trans.translate_expr(&arg))
					}
					let call = trans.builder.ins().call(local_callee, &arg_values);
					assert!(!trans.builder.inst_results(call).is_empty());
				}
				AstNode::FunctionDeclaration(..) => {}
			}
		}

		trans.builder.ins().return_(&[]);

		self.functions = trans.functions;
		self.variables = trans.variables;

		if let Err(errors) = verify_function(&self.ctx.func, self.module.isa()) {
			eprintln!("Verifier errors: {}", errors);
		}

		let id = self
			.module
			.declare_function(
				&format!("main{}", self.counter),
				Linkage::Export,
				&self.ctx.func.signature,
			)
			.map_err(|e| e.to_string())?;

		self
			.module
			.define_function(id, &mut self.ctx)
			.map_err(|e| e.to_string())?;

		self.module.clear_context(&mut self.ctx);

		self.module.finalize_definitions().unwrap();

		Ok(self.module.get_finalized_function(id))
	}

	pub fn renew(&mut self) {
		self.counter += 1;
		self.builder_context = FunctionBuilderContext::new();
	}
}

struct Translator<'a> {
	ty: types::Type,
	builder: FunctionBuilder<'a>,
	variables: HashMap<String, (Variable, Option<Expression>)>,
	functions: HashMap<String, FuncId>,
	module: &'a mut JITModule,
}

impl Translator<'_> {
	pub fn translate_expr(&mut self, expr: &Expression) -> Value {
		match expr {
			Expression::Abs(expression) => {
				let data = self.translate_expr(expression);
				self.builder.ins().fabs(data)
			}
			Expression::Binary(lhs, op, rhs) => {
				let ld = self.translate_expr(lhs);
				let rd = self.translate_expr(rhs);

				match op {
					Token::Add => self.builder.ins().fadd(ld, rd),
					Token::Sub => self.builder.ins().fsub(ld, rd),
					Token::Mul => self.builder.ins().fmul(ld, rd),
					Token::Div => self.builder.ins().fdiv(ld, rd),
					Token::Pow => {
						let arg_values = [ld, rd];
						let mut sig = self.module.make_signature();

						for _arg in &arg_values {
							sig.params.push(AbiParam::new(self.ty));
						}

						sig.returns.push(AbiParam::new(self.ty));

						let callee = self
							.module
							.declare_function("pow", Linkage::Local, &sig)
							.expect("problem declaring function");

						let local_callee = self.module.declare_func_in_func(callee, self.builder.func);

						let call = self.builder.ins().call(local_callee, &arg_values);

						*self.builder.inst_results(call).first().unwrap()
					}
					Token::Rem => {
						let ild = self
							.builder
							.ins()
							.fcvt_to_sint(self.module.target_config().pointer_type(), ld);

						let ird = self
							.builder
							.ins()
							.fcvt_to_sint(self.module.target_config().pointer_type(), rd);

						let rem = self.builder.ins().srem(ild, ird);

						self.builder.ins().fcvt_from_sint(self.ty, rem)
					}
					Token::IsEq => {
						let cmp = self.builder.ins().fcmp(FloatCC::Equal, ld, rd);
						self.builder.ins().fcvt_from_sint(self.ty, cmp)
					}
					Token::NEq => {
						let cmp = self.builder.ins().fcmp(FloatCC::NotEqual, ld, rd);
						self.builder.ins().fcvt_from_sint(self.ty, cmp)
					}
					Token::Lt => {
						let cmp = self.builder.ins().fcmp(FloatCC::LessThan, ld, rd);
						self.builder.ins().fcvt_from_sint(self.ty, cmp)
					}
					Token::LtEq => {
						let cmp = self.builder.ins().fcmp(FloatCC::LessThanOrEqual, ld, rd);
						self.builder.ins().fcvt_from_sint(self.ty, cmp)
					}
					Token::Gt => {
						let cmp = self.builder.ins().fcmp(FloatCC::GreaterThan, ld, rd);
						self.builder.ins().fcvt_from_sint(self.ty, cmp)
					}
					Token::GtEq => {
						let cmp = self.builder.ins().fcmp(FloatCC::GreaterThanOrEqual, ld, rd);
						self.builder.ins().fcvt_from_sint(self.ty, cmp)
					}
					_ => unreachable!("{op:?}"),
				}
			}
			Expression::Branched(condition, then_body, else_body) => {
				let condition_value = self.translate_expr(condition);
				let icondition_value = self
					.builder
					.ins()
					.fcvt_to_sint(self.module.target_config().pointer_type(), condition_value);

				let then_block = self.builder.create_block();
				let else_block = self.builder.create_block();
				let merge_block = self.builder.create_block();

				self.builder.append_block_param(merge_block, self.ty);

				self
					.builder
					.ins()
					.brif(icondition_value, then_block, &[], else_block, &[]);

				self.builder.switch_to_block(then_block);
				self.builder.seal_block(then_block);
				let then_return = self.translate_expr(then_body);

				self.builder.ins().jump(merge_block, &[then_return]);

				self.builder.switch_to_block(else_block);
				self.builder.seal_block(else_block);
				let else_return = self.translate_expr(else_body);

				self.builder.ins().jump(merge_block, &[else_return]);

				self.builder.switch_to_block(merge_block);

				self.builder.seal_block(merge_block);

				let phi = self.builder.block_params(merge_block)[0];

				phi
			}
			Expression::Identifier(ident) => {
				if self.variables.contains_key(ident) {
					self.builder.use_var(self.variables.get(ident).unwrap().0)
				} else {
					let func_ptr = self
						.module
						.get_finalized_function(*self.functions.get(ident).unwrap());

					self.builder.ins().f64const(func_ptr as u64 as f64)
				}
			}
			Expression::Number(n) => self.builder.ins().f64const(*n),
			Expression::FunctionCall(ident, args) => {
				let mut sig = self.module.make_signature();

				for _arg in args {
					sig.params.push(AbiParam::new(self.ty));
				}

				sig.returns.push(AbiParam::new(self.ty));

				let callee = self
					.module
					.declare_function(ident, Linkage::Local, &sig)
					.expect("problem declaring function");

				let local_callee = self.module.declare_func_in_func(callee, self.builder.func);

				let mut arg_values = Vec::new();
				for arg in args {
					arg_values.push(self.translate_expr(arg))
				}
				let call = self.builder.ins().call(local_callee, &arg_values);
				*self.builder.inst_results(call).first().unwrap()
			}
		}
	}
}
