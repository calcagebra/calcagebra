use crate::types::Number;

pub fn int(a: &Number) -> Number {
	Number::Int(a.int())
}

pub fn real(a: &Number) -> Number {
	Number::Real(a.real())
}
