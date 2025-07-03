pub mod io;
pub mod math;
pub mod operators;
pub mod types;

use crate::types::DataType;

// TODO: Add operators in this
/// Returns argument type and return type
pub fn internal_type_map(f: &str) -> (Vec<Vec<DataType>>, DataType) {
	match f {
		"read" => (vec![], DataType::Real),
		"real" => (vec![vec![DataType::Int]], DataType::Real),
		"int" => (vec![vec![DataType::Real]], DataType::Int),
		"print" | "round" | "ceil" | "floor" | "ln" | "log10" | "sin" | "cos" | "tan" | "sqrt"
		| "cbrt" | "graph" => (vec![vec![DataType::Real]], DataType::Real),
		"log" | "nrt" => (
			vec![vec![DataType::Real], vec![DataType::Real]],
			DataType::Real,
		),
		"transpose" | "determinant" | "adj" | "inverse" => {
			(vec![vec![DataType::Matrix]], DataType::Matrix)
		}
		"abs" => (
			vec![vec![
				DataType::Int,
				DataType::Real,
				DataType::Complex,
				DataType::Matrix,
			]],
			DataType::Real,
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
