use std::{collections::HashMap, ops::Range, str::FromStr};

pub type InterpreterContext<'a> = (
	&'a mut HashMap<String, Variable>,
	&'a mut HashMap<String, Function>,
);

use rust_decimal::Decimal;

use crate::{
	errors::{Error, TypeError},
	expr::Expression,
	standardlibrary::{io, iter, math, operators},
	token::Token,
	types::{Data, DataType},
};

#[derive(Debug, Clone)]
pub struct Interpreter {
	pub globals: HashMap<String, Variable>,
	pub functions: HashMap<String, Function>,
}

impl Default for Interpreter {
	fn default() -> Self {
		Self::new()
	}
}

impl Interpreter {
	#[inline(always)]
	pub fn new() -> Self {
		let mut globals = HashMap::new();
		let mut functions = HashMap::new();

		[
			("i", Data::new_img(Decimal::ONE)),
			("pi", Data::new_real(Decimal::PI)),
			("π", Data::new_real(Decimal::PI)),
			("e", Data::new_real(Decimal::E)),
		]
		.map(|(global, data)| globals.insert(global.to_string(), Variable::new(data, true)));

		[
			"print",
			"read",
			"int",
			"real",
			"add",
			"sub",
			"mul",
			"div",
			"pow",
			"rem",
			"is_eq",
			"neq",
			"gt",
			"gteq",
			"lt",
			"lteq",
			"abs",
			"round",
			"ceil",
			"floor",
			"exp",
			"ln",
			"log10",
			"log",
			"sin",
			"sinh",
			"cos",
			"cosh",
			"tan",
			"atan",
			"atan2",
			"sqrt",
			"cbrt",
			"nrt",
			"graph",
			"transpose",
			"determinant",
			"adj",
			"inverse",
			"sum",
			"prod",
			"map",
			"differentiate",
			"quadroot",
		]
		.map(|name| {
			functions.insert(
				name.to_string(),
				Function::STD(STDFunction {
					name: name.to_string(),
				}),
			)
		});

		Self { globals, functions }
	}

	#[inline(always)]
	pub fn interpret(&mut self, ast: Vec<(Expression, Range<usize>)>) -> Result<(), Error> {
		let ctx = &mut (&mut self.globals, &mut self.functions);

		for (expr, range) in ast {
			expr.evaluate(ctx, range)?;
		}

		Ok(())
	}
}

#[derive(Debug, Clone)]
pub struct Variable {
	pub value: Data,
	pub is_global: bool,
}

impl Variable {
	#[inline(always)]
	pub fn new(value: Data, is_global: bool) -> Self {
		Self { value, is_global }
	}
}

#[derive(Debug, Clone)]
pub enum Function {
	UserDefined(UserDefinedFunction),
	STD(STDFunction),
}

impl Function {
	#[inline(always)]
	pub fn execute<'a, 'b>(
		&self,
		ctx: &'a mut InterpreterContext<'b>,
		args: Vec<Data>,
	) -> Result<Data, Error>
	where
		'b: 'a,
	{
		if let Function::UserDefined(user_defined_function) = self {
			user_defined_function.execute(ctx, args)
		} else if let Function::STD(stdfunction) = self {
			stdfunction.execute(ctx, args)
		} else {
			unreachable!()
		}
	}

	#[inline(always)]
	pub fn differentiate<'a, 'b>(
		&self,
		a: &Data,
		args: &[(Expression, Range<usize>)],
		ctx: &'a mut InterpreterContext<'b>,
	) -> Result<Data, Error>
	where
		'b: 'a,
	{
		if let Function::UserDefined(user_defined_function) = self {
			user_defined_function.differentiate(a, ctx)
		} else if let Function::STD(stdfunction) = self {
			stdfunction.differentiate(a, args, ctx)
		} else {
			unreachable!()
		}
	}
}

#[derive(Debug, Clone)]
pub struct UserDefinedFunction {
	pub params: Vec<(String, DataType)>,
	pub return_type: DataType,
	pub code: Expression,
	pub range: Range<usize>,
}

impl UserDefinedFunction {
	#[inline(always)]
	pub fn execute<'a, 'b>(
		&self,
		ctx: &'a mut InterpreterContext<'b>,
		mut args: Vec<Data>,
	) -> Result<Data, Error>
	where
		'b: 'a,
	{
		let mut param_names = vec![];

		for (i, (arg, numbertype)) in self.params.iter().enumerate() {
			let r = args.remove(i);

			if r.ty() != *numbertype {
				return Err(TypeError::new(*numbertype, r.ty(), 0..0).to_error());
			}

			param_names.push((
				arg.to_string(),
				if ctx.0.get(arg).is_some() {
					Some(ctx.0.get(arg).unwrap().clone())
				} else {
					None
				},
			));

			ctx.0.insert(arg.to_string(), Variable::new(r, false));
		}

		let data = self.code.clone().evaluate(ctx, self.range.clone());

		for (name, value) in param_names {
			if let Some(value) = value {
				ctx.0.insert(name, value);
			} else {
				ctx.0.remove(&name);
			}
		}

		data
	}

	#[inline(always)]
	pub fn differentiate<'a, 'b>(
		&self,
		wrt: &Data,
		ctx: &'a mut InterpreterContext<'b>,
	) -> Result<Data, Error>
	where
		'b: 'a,
	{
		Ok(Data::Expression(self.code.differentiate(wrt, ctx)?))
	}
}

#[derive(Debug, Clone)]
pub struct STDFunction {
	pub name: String,
}

impl STDFunction {
	#[inline(always)]
	pub fn execute<'a, 'b>(
		&self,
		ctx: &'a mut InterpreterContext<'b>,
		args: Vec<Data>,
	) -> Result<Data, Error>
	where
		'b: 'a,
	{
		Ok(match self.name.as_str() {
			"print" => io::print(args),
			"read" => io::read(ctx)?,
			"add" => operators::add(&args[0], &args[1]),
			"sub" => operators::sub(&args[0], &args[1]),
			"mul" => operators::mul(&args[0], &args[1]),
			"div" => operators::div(&args[0], &args[1]),
			"pow" => operators::pow(&args[0], &args[1]),
			"rem" => operators::rem(&args[0], &args[1]),
			"is_eq" => operators::is_eq(&args[0], &args[1]),
			"neq" => operators::neq(&args[0], &args[1]),
			"gt" => operators::gt(&args[0], &args[1]),
			"gteq" => operators::gteq(&args[0], &args[1]),
			"lt" => operators::lt(&args[0], &args[1]),
			"lteq" => operators::lteq(&args[0], &args[1]),
			"abs" => math::abs(&args[0]),
			"round" => math::round(&args[0]),
			"ceil" => math::ceil(&args[0]),
			"floor" => math::floor(&args[0]),
			"exp" => math::exp(&args[0]),
			"ln" => math::ln(&args[0]),
			"log10" => math::log10(&args[0]),
			"log" => math::log(&args[0], &args[1]),
			"sin" => math::sin(&args[0]),
			"sinh" => math::sinh(&args[0]),
			"cos" => math::cos(&args[0]),
			"cosh" => math::cosh(&args[0]),
			"tan" => math::tan(&args[0]),
			"atan" => math::atan(&args[0]),
			"atan2" => math::atan2(&args[0], &args[0]),
			"sqrt" => math::sqrt(&args[0]),
			"nrt" => math::nrt(&args[0], &args[1]),
			"transpose" => math::transpose(&args[0]),
			"determinant" => math::determinant(&args[0]),
			"adj" => math::adj(&args[0]),
			"inverse" => math::inverse(&args[0]),
			"graph" => math::graph(&args[0], ctx)?,
			"sum" => math::sum(&args[0], &args[1], &args[2], ctx)?,
			"prod" => math::prod(&args[0], &args[1], &args[2], ctx)?,
			"map" => iter::map(&args[0], &args[1], ctx)?,
			"differentiate" => math::differentiate(&args[0], &args[1], ctx)?,
			"quadroot" => math::quadroot(&args[0], ctx)?,
			_ => unreachable!(),
		})
	}

	#[inline(always)]
	pub fn differentiate<'a, 'b>(
		&self,
		wrt: &Data,
		args: &[(Expression, Range<usize>)],
		_ctx: &'a mut InterpreterContext<'b>,
	) -> Result<Data, Error>
	where
		'b: 'a,
	{
		let Data::Ident(_) = wrt else {
			return Err(Error::LogicError(
				"expected variable to differentiate".to_string(),
			));
		};

		Ok(match self.name.as_str() {
			"exp" => Data::Expression(Expression::FunctionCall("exp".to_string(), args.to_vec())),
			"ln" => Data::Expression(Expression::Binary(
				Box::new(Expression::Float(Decimal::ONE)),
				Token::Div,
				Box::new(args[0].0.clone()),
			)),
			"log10" => Data::Expression(Expression::Binary(
				Box::new(Expression::Float(
					Decimal::from_str("0.4342944819032518276511289188").unwrap(),
				)),
				Token::Div,
				Box::new(args[0].0.clone()),
			)),
			"sin" => Data::Expression(Expression::FunctionCall("cos".to_string(), args.to_vec())),
			"cos" => Data::Expression(Expression::Binary(
				Box::new(Expression::Float(Decimal::NEGATIVE_ONE)),
				Token::Mul,
				Box::new(Expression::FunctionCall("sin".to_string(), args.to_vec())),
			)),
			"sinh" => Data::Expression(Expression::FunctionCall("cosh".to_string(), args.to_vec())),
			"cosh" => Data::Expression(Expression::FunctionCall("sinh".to_string(), args.to_vec())),
			"tan" => Data::Expression(Expression::Binary(
				Box::new(Expression::Float(Decimal::ONE)),
				Token::Div,
				Box::new(Expression::Binary(
					Box::new(Expression::FunctionCall("cos".to_string(), args.to_vec())),
					Token::Pow,
					Box::new(Expression::Float(Decimal::TWO)),
				)),
			)),
			"atan" => Data::Expression(Expression::Binary(
				Box::new(Expression::Float(Decimal::ONE)),
				Token::Div,
				Box::new(Expression::Binary(
					Box::new(Expression::Float(Decimal::ONE)),
					Token::Add,
					Box::new(Expression::Binary(
						Box::new(args[0].0.clone()),
						Token::Pow,
						Box::new(Expression::Float(Decimal::TWO)),
					)),
				)),
			)),
			"sqrt" => Data::Expression(Expression::Binary(
				Box::new(Expression::Float(Decimal::ONE)),
				Token::Div,
				Box::new(Expression::Binary(
					Box::new(Expression::Float(Decimal::TWO)),
					Token::Mul,
					Box::new(Expression::FunctionCall("sqrt".to_string(), args.to_vec())),
				)),
			)),
			_ => {
				return Err(Error::LogicError(
					"attempt to differentiate standard library function which has no derivative".to_string(),
				));
			}
		})
	}
}
