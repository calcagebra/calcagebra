use crate::{errors::Error, interpreter::InterpreterContext, types::Data};

pub fn map(f: &Data, a: &Data, ctx: &mut InterpreterContext) -> Result<Data, Error> {
	let Data::Matrix(matrix) = a else {
		unreachable!()
	};

	let Data::Ident(g) = f else { unreachable!() };

	let func = ctx.1.get(g).unwrap().clone();

	let mut matrix_data = vec![];

	for row in matrix {
		let mut row_data = vec![];
		for element in row {
			let data = func.execute(ctx, vec![element.clone()])?;

			row_data.push(data);
		}
		matrix_data.push(row_data);
	}

	Ok(Data::Matrix(matrix_data))
}
