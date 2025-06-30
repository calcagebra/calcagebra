pub mod io;
pub mod math;
pub mod operands;
pub mod types;

use crate::ast::Expression;
use crate::interpreter::Interpreter;
use crate::types::{Number, NumberType};

// TODO: Add operands in this
/// Returns argument type and return type
pub fn internal_type_map(f: &str) -> (Vec<Vec<NumberType>>, NumberType) {
	match f {
		"read" => (vec![], NumberType::Real),
		"real" => (vec![vec![NumberType::Int]], NumberType::Real),
		"int" => (vec![vec![NumberType::Real]], NumberType::Int),
		"print" | "round" | "ceil" | "floor" | "ln" | "log10" | "sin" | "cos" | "tan" | "sqrt"
		| "cbrt" | "graph" => (vec![vec![NumberType::Real]], NumberType::Real),
		"log" | "nrt" => (
			vec![vec![NumberType::Real], vec![NumberType::Real]],
			NumberType::Real,
		),
		"transpose" | "determinant" | "adj" | "inverse" => {
			(vec![vec![NumberType::Matrix]], NumberType::Matrix)
		}
		"abs" => (
			vec![vec![
				NumberType::Int,
				NumberType::Real,
				NumberType::Complex,
				NumberType::Matrix,
			]],
			NumberType::Real,
		),
		_ => unimplemented!("type map not implemented for: {f}"),
	}
}

pub fn is_std(f: &str) -> bool {
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
		"ln",
		"log10",
		"log",
		"sin",
		"cos",
		"tan",
		"sqrt",
		"cbrt",
		"nrt",
		"graph",
		"transpose",
		"determinant",
		"adj",
		"inverse",
	]
	.contains(&f)
}

pub fn needs_ctx(f: &str) -> bool {
	["graph"].contains(&f)
}

pub fn call(f: &str, args: Vec<Number>) -> Number {
	match f {
		"print" => io::print(args),
		"read" => io::read(),
		"int" => types::int(args),
		"real" => types::real(args),
		"add" => operands::add(&args[0], &args[1]),
		"sub" => operands::sub(&args[0], &args[1]),
		"mul" => operands::mul(&args[0], &args[1]),
		"div" => operands::div(&args[0], &args[1]),
		"pow" => operands::pow(&args[0], &args[1]),
		"rem" => operands::rem(&args[0], &args[1]),
		"is_eq" => operands::is_eq(&args[0], &args[1]),
		"neq" => operands::neq(&args[0], &args[1]),
		"gt" => operands::gt(&args[0], &args[1]),
		"gteq" => operands::gteq(&args[0], &args[1]),
		"lt" => operands::lt(&args[0], &args[1]),
		"lteq" => operands::lteq(&args[0], &args[1]),
		"abs" => math::abs(args),
		"round" => math::round(args),
		"ceil" => math::ceil(args),
		"floor" => math::floor(args),
		"ln" => math::ln(args),
		"log10" => math::log10(args),
		"log" => math::log(args),
		"sin" => math::sin(args),
		"cos" => math::cos(args),
		"tan" => math::tan(args),
		"sqrt" => math::sqrt(args),
		"cbrt" => math::cbrt(args),
		"nrt" => math::nrt(args),
		"transpose" => math::transpose(args),
		"determinant" => math::determinant(args),
		"adj" => math::adj(args),
		"inverse" => math::inverse(args),
		_ => unreachable!(),
	}
}

pub fn ctx_call(f: &str, args: Vec<Expression>, interpreter: &mut Interpreter) -> Number {
	match f {
		"graph" => math::graph(args, interpreter),
		_ => unreachable!(),
	}
}
