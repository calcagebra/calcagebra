use crate::types::DataType;

pub mod io;
pub mod iter;
pub mod math;
pub mod operators;

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

// TODO: Add operators in this
/// Returns argument type and return type
pub fn internal_type_map(f: &str) -> (Vec<Vec<DataType>>, DataType) {
	match f {
		"read" => (vec![], DataType::Number),
		"real" => (vec![vec![DataType::Number]], DataType::Number),
		"int" => (vec![vec![DataType::Number]], DataType::Number),
		"print" | "round" | "ceil" | "floor" | "ln" | "log10" | "sin" | "cos" | "tan" | "sqrt"
		| "cbrt" | "graph" => (vec![vec![DataType::Number]], DataType::Number),
		"log" | "nrt" => (
			vec![vec![DataType::Number], vec![DataType::Number]],
			DataType::Number,
		),
		"transpose" | "determinant" | "adj" | "inverse" => {
			(vec![vec![DataType::Matrix]], DataType::Matrix)
		}
		"abs" => (
			vec![vec![DataType::Number, DataType::Matrix]],
			DataType::Number,
		),
		_ => unimplemented!("type map not implemented for: {f}"),
	}
}
