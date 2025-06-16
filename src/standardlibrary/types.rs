use crate::types::Number;

pub fn int(a: Vec<Number>) -> Number {
	Number::Int(a[0].int())
}

pub fn real(a: Vec<Number>) -> Number {
	Number::Real(a[0].real())
}
