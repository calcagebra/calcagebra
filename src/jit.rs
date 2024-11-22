use std::collections::HashMap;

use codegen::{ir, verify_function};
use cranelift::{
	jit::{JITBuilder, JITModule},
	module::{default_libcall_names, DataDescription, Linkage, Module},
	native::builder,
	prelude::*,
};

use crate::{
	ast::{AstNode, Expression},
	token::Token,
};

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
unsafe extern "C" fn print_std(value: f64) -> f64 {
	println!("{}", value);
	0.0
}

unsafe extern "C" fn pow_std(a: f64, b: f64) -> f64 {
	a.powf(b)
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

		let print_addr = print_std as *const u8;
		builder.symbol("print", print_addr);
		let pow_addr: *const u8 = pow_std as *const u8;
		builder.symbol("pow", pow_addr);

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
	pub fn translate_expr(&mut self, expr: Expression) -> Value {
		match expr {
			Expression::Abs(expression) => {
				let data = self.translate_expr(*expression);
				self.builder.ins().fabs(data)
			}
			Expression::Binary(lhs, op, rhs) => {
				let ld = self.translate_expr(*lhs);
				let rd = self.translate_expr(*rhs);

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

						let local_callee: ir::FuncRef =
							self.module.declare_func_in_func(callee, self.builder.func);

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
				let condition_value = self.translate_expr(*condition);
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
				let then_return = self.translate_expr(*then_body);

				self.builder.ins().jump(merge_block, &[then_return]);

				self.builder.switch_to_block(else_block);
				self.builder.seal_block(else_block);
				let else_return = self.translate_expr(*else_body);

				self.builder.ins().jump(merge_block, &[else_return]);

				self.builder.switch_to_block(merge_block);

				self.builder.seal_block(merge_block);

				let phi = self.builder.block_params(merge_block)[0];

				phi
			}
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
