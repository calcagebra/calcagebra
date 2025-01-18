use std::collections::{hash_map::Entry, HashMap};

use codegen::verify_function;
use cranelift::{
	jit::{JITBuilder, JITModule},
	module::{default_libcall_names, FuncId, Linkage, Module},
	native::builder,
	prelude::*,
};

use crate::{
	ast::{AstNode, AstType, Expression},
	standardlibrary::*,
	token::Token,
};

#[derive(Clone, Debug)]
pub struct VariableWrapper {
	pub id: Variable,
	pub code: Option<Expression>, // Code used to initialise this variable with a value, only used in REPL
	pub datatype: AstType,
}

#[derive(Clone, Debug)]
pub struct FunctionWrapper {
	pub id: FuncId,
	pub params: Vec<AstType>,
	pub return_type: AstType,
}

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

	variables: HashMap<String, VariableWrapper>,
	functions: HashMap<String, FunctionWrapper>,

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

		// TYPES
		builder.symbol("int", int as *const u8);
		builder.symbol("float", float as *const u8);

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
	pub fn execute(&mut self, ast: Vec<AstNode>, debug: bool) -> Result<*const u8, String> {
		let mut filtered_ast = vec![];
		let mut functions = self.functions.clone();

		for node in ast {
			match node {
				AstNode::FunctionDeclaration(name, params, return_type, expr) => {
					for p in &params {
						self
							.ctx
							.func
							.signature
							.params
							.push(AbiParam::new(p.1.resolve()));
					}

					self
						.ctx
						.func
						.signature
						.returns
						.push(AbiParam::new(return_type.resolve()));

					let mut builder = FunctionBuilder::new(&mut self.ctx.func, &mut self.builder_context);
					let entry_block = builder.create_block();

					builder.append_block_params_for_function_params(entry_block);
					builder.switch_to_block(entry_block);
					builder.seal_block(entry_block);

					let mut variables = HashMap::new();
					let mut index = 0;
					let param_types = params.iter().map(|param| param.1).collect::<Vec<AstType>>();

					for param in params {
						let val = builder.block_params(entry_block)[index];
						let var = Variable::new(index);
						if let Entry::Vacant(e) = variables.entry(param.0) {
							e.insert(VariableWrapper {
								id: var,
								code: None,
								datatype: param.1,
							});
							builder.declare_var(var, param.1.resolve());
							index += 1;
						}
						builder.def_var(var, val);
					}

					let mut trans = Translator {
						builder,
						variables,
						functions: HashMap::new(),
						module: &mut self.module,
					};

					let (mut ret_value, datatype) = trans.translate_expr(&expr);

					if datatype != return_type {
						ret_value = match (datatype, return_type) {
							(AstType::Int, AstType::Float) => {
								trans.builder.ins().fcvt_from_sint(types::F64, ret_value)
							}
							(AstType::Float, AstType::Int) => {
								trans.builder.ins().fcvt_to_sint(types::I64, ret_value)
							}
							_ => unreachable!(),
						}
					}

					trans.builder.ins().return_(&[ret_value]);

					trans.builder.finalize();

					if debug {
						println!("{}", self.ctx.func);
					}

					if let Err(errors) = verify_function(&self.ctx.func, self.module.isa()) {
						eprintln!("Verifier errors in function: {}", errors);
					}

					let id = self
						.module
						.declare_function(&name, Linkage::Export, &self.ctx.func.signature)
						.map_err(|e| e.to_string())?;

					functions.insert(
						name,
						FunctionWrapper {
							id,
							params: param_types,
							return_type,
						},
					);

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
			builder,
			variables: HashMap::new(),
			functions,
			module: &mut self.module,
		};

		for (name, vw) in &self.variables {
			let (val, _) = trans.translate_expr(vw.code.as_ref().unwrap());

			trans.variables.insert(name.to_string(), vw.clone());
			trans.builder.declare_var(vw.id, vw.datatype.resolve());

			trans.builder.def_var(vw.id, val);
		}

		for node in filtered_ast {
			match node {
				AstNode::Assignment((name, datatype), expression) => {
					let idx = trans.variables.len();
					let var = Variable::new(idx);

					let (val, _) = trans.translate_expr(&expression);

					trans.variables.insert(
						name.to_string(),
						VariableWrapper {
							id: var,
							code: Some(expression),
							datatype,
						},
					);
					trans.builder.declare_var(var, datatype.resolve());

					trans.builder.def_var(var, val);
				}
				AstNode::FunctionCall(ident, args) => {
					let mut sig = trans.module.make_signature();

					if let Some(f) = trans.functions.get(&ident) {
						for (i, _arg) in args.iter().enumerate() {
							sig.params.push(AbiParam::new(f.params[i].resolve()));
						}

						sig.returns.push(AbiParam::new(f.return_type.resolve()));
					} else {
						let type_map = type_map(&ident);

						for (i, _arg) in args.iter().enumerate() {
							sig.params.push(AbiParam::new(type_map.0[i].resolve()));
						}

						sig.returns.push(AbiParam::new(type_map.1.resolve()));
					};

					let callee = trans
						.module
						.declare_function(&ident, Linkage::Local, &sig)
						.expect("problem declaring function");

					let local_callee = trans
						.module
						.declare_func_in_func(callee, trans.builder.func);

					let mut arg_values = Vec::new();

					for arg in args {
						arg_values.push(trans.translate_expr(&arg).0)
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

		if debug {
			println!("{}", self.ctx.func);
		}

		if let Err(errors) = verify_function(&self.ctx.func, self.module.isa()) {
			eprintln!("Verifier errors in IR: {}", errors);
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
	builder: FunctionBuilder<'a>,
	variables: HashMap<String, VariableWrapper>,
	functions: HashMap<String, FunctionWrapper>,
	module: &'a mut JITModule,
}

impl Translator<'_> {
	pub fn translate_expr(&mut self, expr: &Expression) -> (Value, AstType) {
		match expr {
			Expression::Abs(expression) => {
				let (data, datatype) = self.translate_expr(expression);
				(self.builder.ins().fabs(data), datatype)
			}
			Expression::Binary(lhs, op, rhs) => {
				let (ld, lt) = self.translate_expr(lhs);
				let (rd, rt) = self.translate_expr(rhs);

				let mut sorted_types = [lt, rt];
				sorted_types.sort();
				let return_type = sorted_types[0];

				(
					match (op, return_type) {
						(Token::Add, AstType::Int) => self.builder.ins().iadd(ld, rd),
						(Token::Add, AstType::Float) => self.builder.ins().fadd(ld, rd),
						(Token::Sub, AstType::Int) => self.builder.ins().isub(ld, rd),
						(Token::Sub, AstType::Float) => self.builder.ins().fsub(ld, rd),
						(Token::Mul, AstType::Int) => self.builder.ins().imul(ld, rd),
						(Token::Mul, AstType::Float) => self.builder.ins().fmul(ld, rd),
						(Token::Div, AstType::Int) => {
							let value = self.builder.ins().fdiv(ld, rd);
							self
								.builder
								.ins()
								.fcvt_to_sint(self.module.target_config().pointer_type(), value)
						}
						(Token::Div, AstType::Float) => self.builder.ins().fdiv(ld, rd),
						(Token::Pow, t) => {
							let ild = if lt == AstType::Int {
								self.builder.ins().fcvt_from_sint(types::F64, ld)
							} else {
								ld
							};

							let ird = if rt == AstType::Int {
								self.builder.ins().fcvt_from_sint(types::F64, rd)
							} else {
								rd
							};

							let arg_values = [ild, ird];
							let mut sig = self.module.make_signature();

							for _arg in &arg_values {
								sig.params.push(AbiParam::new(types::F64));
							}

							sig.returns.push(AbiParam::new(types::F64));

							let callee = self
								.module
								.declare_function("pow", Linkage::Local, &sig)
								.expect("problem declaring function");

							let local_callee = self.module.declare_func_in_func(callee, self.builder.func);

							let call = self.builder.ins().call(local_callee, &arg_values);

							let value = *self.builder.inst_results(call).first().unwrap();

							match t {
								AstType::Int => self
									.builder
									.ins()
									.fcvt_to_sint(self.module.target_config().pointer_type(), value),
								AstType::Float => value,
							}
						}
						(Token::Rem, t) => {
							let ild = self
								.builder
								.ins()
								.fcvt_to_sint(self.module.target_config().pointer_type(), ld);

							let ird = self
								.builder
								.ins()
								.fcvt_to_sint(self.module.target_config().pointer_type(), rd);

							let rem = self.builder.ins().srem(ild, ird);

							let value = self.builder.ins().fcvt_from_sint(types::F64, rem);

							match t {
								AstType::Int => value,
								AstType::Float => self.builder.ins().fcvt_from_sint(types::F64, value),
							}
						}
						(Token::IsEq, _) => {
							let cmp = self.builder.ins().fcmp(FloatCC::Equal, ld, rd);
							self.builder.ins().fcvt_from_sint(types::F64, cmp)
						}
						(Token::NEq, _) => {
							let cmp = self.builder.ins().fcmp(FloatCC::NotEqual, ld, rd);
							self.builder.ins().fcvt_from_sint(types::F64, cmp)
						}
						(Token::Lt, _) => {
							let cmp = self.builder.ins().fcmp(FloatCC::LessThan, ld, rd);
							self.builder.ins().fcvt_from_sint(types::F64, cmp)
						}
						(Token::LtEq, _) => {
							let cmp = self.builder.ins().fcmp(FloatCC::LessThanOrEqual, ld, rd);
							self.builder.ins().fcvt_from_sint(types::F64, cmp)
						}
						(Token::Gt, _) => {
							let cmp = self.builder.ins().fcmp(FloatCC::GreaterThan, ld, rd);
							self.builder.ins().fcvt_from_sint(types::F64, cmp)
						}
						(Token::GtEq, _) => {
							let cmp = self.builder.ins().fcmp(FloatCC::GreaterThanOrEqual, ld, rd);
							self.builder.ins().fcvt_from_sint(types::F64, cmp)
						}
						_ => unreachable!("{op:?}"),
					},
					return_type,
				)
			}
			Expression::Branched(condition, then_body, else_body) => {
				let (condition_value, _) = self.translate_expr(condition);
				let icondition_value = self
					.builder
					.ins()
					.fcvt_to_sint(self.module.target_config().pointer_type(), condition_value);

				let then_block = self.builder.create_block();
				let else_block = self.builder.create_block();
				let merge_block = self.builder.create_block();

				self.builder.append_block_param(merge_block, types::F64);

				self
					.builder
					.ins()
					.brif(icondition_value, then_block, &[], else_block, &[]);

				self.builder.switch_to_block(then_block);
				self.builder.seal_block(then_block);
				let (then_return, tr) = self.translate_expr(then_body);

				self.builder.ins().jump(merge_block, &[then_return]);

				self.builder.switch_to_block(else_block);
				self.builder.seal_block(else_block);
				let (else_return, er) = self.translate_expr(else_body);

				self.builder.ins().jump(merge_block, &[else_return]);

				self.builder.switch_to_block(merge_block);

				self.builder.seal_block(merge_block);

				let phi = self.builder.block_params(merge_block)[0];

				assert!(er == tr);

				(phi, tr)
			}
			Expression::Identifier(ident) => {
				if self.variables.contains_key(ident) {
					let var = self.variables.get(ident).unwrap();
					(self.builder.use_var(var.id), var.datatype)
				} else {
					let func_ptr = self
						.module
						.get_finalized_function(self.functions.get(ident).unwrap().id);

					(
						self.builder.ins().f64const(func_ptr as u64 as f64),
						AstType::Float,
					)
				}
			}
			Expression::Integer(n) => (
				self.builder.ins().iconst(AstType::Int.resolve(), *n),
				AstType::Int,
			),
			Expression::Float(n) => (self.builder.ins().f64const(*n), AstType::Float),
			Expression::FunctionCall(ident, args) => {
				let mut sig = self.module.make_signature();

				let return_type = if let Some(f) = self.functions.get(ident) {
					for (i, _arg) in args.iter().enumerate() {
						sig.params.push(AbiParam::new(f.params[i].resolve()));
					}

					sig.returns.push(AbiParam::new(f.return_type.resolve()));

					f.return_type
				} else {
					let type_map = type_map(ident);

					for (i, _arg) in args.iter().enumerate() {
						sig.params.push(AbiParam::new(type_map.0[i].resolve()));
					}

					sig.returns.push(AbiParam::new(type_map.1.resolve()));

					type_map.1
				};

				let callee = self
					.module
					.declare_function(ident, Linkage::Local, &sig)
					.expect("problem declaring function");

				let local_callee = self.module.declare_func_in_func(callee, self.builder.func);

				let mut arg_values = Vec::new();
				for arg in args {
					arg_values.push(self.translate_expr(arg).0)
				}
				let call = self.builder.ins().call(local_callee, &arg_values);
				(
					*self.builder.inst_results(call).first().unwrap(),
					return_type,
				)
			}
		}
	}
}
