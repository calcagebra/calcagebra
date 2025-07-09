use std::{iter::Peekable, ops::Range, slice::Iter};

use crate::{
	errors::{EOLError, Error, SyntaxError, TypeError},
	expr::Expression,
	token::{Token, TokenInfo},
	types::DataType,
};

pub struct Parser<'a> {
	tokens: &'a Vec<Vec<TokenInfo>>,
}

impl<'a> Parser<'a> {
	#[inline(always)]
	pub fn new(tokens: &'a Vec<Vec<TokenInfo>>) -> Self {
		Self { tokens }
	}

	#[inline(always)]
	pub fn ast(&self) -> Result<Vec<(Expression, Range<usize>)>, Error> {
		let mut ast = vec![];
		let lines = self.tokens;

		for line in lines {
			let mut tokens = line.iter().peekable();

			let (expr, range) = self.parser(&mut tokens, 0)?;

			ast.push((expr, range));
		}

		Ok(ast)
	}

	#[inline(always)]
	pub fn parser<'b>(
		&'b self,
		tokens: &mut Peekable<Iter<'b, TokenInfo>>,
		prec: u16,
	) -> Result<(Expression, Range<usize>), Error> {
		let tokeninfo = &tokens.next();

		if tokeninfo.is_none() {
			return Err(Error::LogicError(
				"expression parser did not find any tokens to parse".to_string(),
			));
		}

		let tokeninfo = &tokeninfo.unwrap();

		let token = &tokeninfo.token;
		let mut expr: Option<Expression> = None;

		let start = tokeninfo.range.start;
		let mut end = tokeninfo.range.end;

		match token {
			Token::Let => {
				let mut datatype = None;

				let mut next_token = tokens.next();

				let name = if next_token.is_some()
					&& let Token::Ident(name) = &next_token.unwrap().token
				{
					name
				} else {
					return Err(
						SyntaxError::new(Token::Ident("ident".to_string()), token.clone(), start..end)
							.to_error(),
					);
				};

				next_token = tokens.next();

				if next_token.is_some() && next_token.unwrap().token == Token::Colon {
					next_token = tokens.next();

					if next_token.is_some()
						&& let Token::Ident(ident) = &next_token.unwrap().token
					{
						datatype = Some(DataType::parse(ident));
						next_token = tokens.next();
					} else {
						let tokeninfo = next_token.unwrap();

						return Err(
							SyntaxError::new(
								Token::Ident("ident".to_string()),
								tokeninfo.token.clone(),
								tokeninfo.range.clone(),
							)
							.to_error(),
						);
					}
				}

				if next_token.is_none() || Token::Eq != next_token.unwrap().token {
					let next_token = next_token.unwrap();

					return Err(
						SyntaxError::new(
							Token::Eq,
							next_token.token.clone(),
							next_token.range.clone(),
						)
						.to_error(),
					);
				}

				let exp;
				let range;

				(exp, range) = self.parser(tokens, 0)?;

				let expr_type = exp.infer_datatype();

				if datatype.is_none() {
					datatype = expr_type
				}

				if let Some(expression_type) = expr_type {
					if expr_type.unwrap() != datatype.unwrap() {
						return Err(TypeError::new(datatype.unwrap(), expression_type, range).to_error());
					}
				}

				expr = Some(Expression::Assignment(
					(name.to_string(), datatype),
					Box::new(exp),
				));
				end = range.end;
			}
			Token::Fn => {
				let next_token = tokens.next();

				let name = if next_token.is_some()
					&& let Token::Ident(name) = &next_token.unwrap().token
				{
					name
				} else {
					return Err(
						SyntaxError::new(Token::Ident("ident".to_string()), token.clone(), start..end)
							.to_error(),
					);
				};

				let next_token = tokens.next();

				if next_token.is_none() || Token::LParen != next_token.unwrap().token {
					let next_token = next_token.unwrap();

					return Err(
						SyntaxError::new(
							Token::Eq,
							next_token.token.clone(),
							next_token.range.clone(),
						)
						.to_error(),
					);
				}

				let mut args = vec![];

				loop {
					let token = tokens.next();

					if token.is_none() {
						break;
					}

					let token = token.unwrap();

					if Token::RParen == token.token {
						break;
					}

					let mut datatype = Some(DataType::Number);

					let next_token = tokens.next();

					if next_token.is_some() && next_token.unwrap().token == Token::Colon {
						let next_token = tokens.next();

						if next_token.is_some()
							&& let Token::Ident(ident) = &next_token.unwrap().token
						{
							datatype = Some(DataType::parse(ident))
						} else {
							let tokeninfo = tokens.next().unwrap();

							return Err(
								SyntaxError::new(
									Token::Ident("ident".to_string()),
									tokeninfo.token.clone(),
									tokeninfo.range.clone(),
								)
								.to_error(),
							);
						}
					}

					match &token.token {
						Token::Ident(i) => args.push((i.to_string(), datatype.unwrap())),
						Token::Comma => {}
						_ => unreachable!(),
					};

					if Token::RParen == next_token.unwrap().token {
						break;
					}
				}

				let mut return_type = Some(DataType::Number);

				let mut next_token = tokens.next();

				if next_token.is_some() && next_token.unwrap().token == Token::Colon {
					next_token = tokens.next();

					if next_token.is_some()
						&& let Token::Ident(ident) = &next_token.unwrap().token
					{
						return_type = Some(DataType::parse(ident));
						next_token = tokens.next();
					} else {
						let tokeninfo = tokens.next().unwrap();

						return Err(
							SyntaxError::new(
								Token::Ident("ident".to_string()),
								tokeninfo.token.clone(),
								tokeninfo.range.clone(),
							)
							.to_error(),
						);
					}
				}

				if next_token.is_none() || Token::Eq != next_token.unwrap().token {
					let next_token = next_token.unwrap();

					return Err(
						SyntaxError::new(
							Token::Eq,
							next_token.token.clone(),
							next_token.range.clone(),
						)
						.to_error(),
					);
				}

				let exp;
				let range;

				(exp, range) = self.parser(tokens, 0)?;

				let expr_type = exp.infer_datatype();

				if let Some(expression_type) = expr_type {
					if expression_type != return_type.unwrap() {
						return Err(TypeError::new(return_type.unwrap(), expression_type, range).to_error());
					}
				}

				expr = Some(Expression::FunctionDeclaration(
					name.to_string(),
					args,
					return_type.unwrap(),
					Box::new(exp),
					start..end,
				));
				end = range.end;
			}
			Token::Ident(i) => {
				// An identifier can either be a function call, in multiplication with a mod
				// or simply an identifier, eg read(), a|b|, c
				let peeked_token = tokens.peek();
				if peeked_token.is_some()
					&& self.infix_binding_power(&peeked_token.unwrap().token) == (0, 0)
					&& ![Token::RParen, Token::Abs].contains(&peeked_token.unwrap().token)
				{
					let exp;

					(exp, end) = self.parse_fn(tokens, i.clone())?;

					expr = Some(exp)
				} else {
					end = tokeninfo.range.end;
					expr = Some(Expression::Identifier(i.to_string()))
				};
			}
			Token::LParen => {
				let exp;
				let range;

				(exp, range) = self.parser(tokens, 0)?;

				end = range.end;
				expr = Some(exp);
				tokens.next();
			}
			Token::LSquare => {
				let mut matrix = vec![];

				let mut row = vec![];

				let mut row_tokens: Vec<TokenInfo> = vec![];

				loop {
					let t = tokens.peek();

					if t.is_none() {
						break;
					}

					let t = tokens.next().unwrap();

					if t.token == Token::RSquare {
						if !row_tokens.is_empty() {
							let exp;

							(exp, _) = self.parser(&mut row_tokens.iter().peekable(), 0)?;

							row.push(exp);
						}

						end = t.range.end;
						matrix.push(row);
						break;
					}

					if t.token == Token::Semi {
						if !row_tokens.is_empty() {
							let exp;

							(exp, _) = self.parser(&mut row_tokens.iter().peekable(), 0)?;

							row.push(exp);
							row_tokens.clear();
						}
						end = t.range.end;
						matrix.push(row.clone());
						row.clear();
						continue;
					}

					if t.token == Token::Comma {
						let exp;

						(exp, _) = self.parser(&mut row_tokens.iter().peekable(), 0)?;

						row.push(exp);
						row_tokens.clear();
						continue;
					}

					end = t.range.end;
					row_tokens.push((*t).clone());
				}

				expr = Some(Expression::Matrix(matrix));
			}
			Token::Abs => {
				let exp;
				let range;

				(exp, range) = self.parser(tokens, 0)?;

				end = range.end;
				expr = Some(Expression::Abs(Box::new(exp)));
				tokens.next();
			}
			Token::If => {
				let exp;

				(exp, end) = self.parse_if(tokens)?;

				expr = Some(exp);
			}
			Token::Sub => {
				if tokens.peek().is_some()
					&& let Token::Float(i) = tokens.peek().unwrap().token
				{
					expr = Some(Expression::Float(-i));
					end = tokens.next().unwrap().range.end;
				} else {
					end = tokeninfo.range.end;
				}
			}
			Token::Float(n) => {
				expr = Some(Expression::Float(*n));
				end = tokeninfo.range.end;
			}
			_ => {
				end = tokeninfo.range.end;
			}
		};

		loop {
			let op = tokens.peek();

			if op.is_none() || [Token::RParen, Token::Abs].contains(&op.unwrap().token) {
				break;
			}

			let (lbp, rbp) = self.infix_binding_power(&op.unwrap().token);

			if lbp < prec {
				break;
			}

			let op = tokens.next().unwrap();

			let rhs;
			let range;

			(rhs, range) = match self.parser(tokens, rbp) {
				Ok(t) => t,
				Err(t) => match t {
					Error::SyntaxError(..) | Error::TypeError(..) | Error::EOLError(..) => {
						return Err(t);
					}
					Error::LogicError(..) => {
						return Err(EOLError::new(end..end + 1).to_error());
					}
				},
			};

			end = range.end;
			expr = Some(Expression::Binary(
				Box::new(expr.unwrap()),
				op.token.clone(),
				Box::new(rhs),
			));
		}

		if expr.is_none() {
			return Err(EOLError::new(end..end + 1).to_error());
		}

		Ok((expr.unwrap(), start..end))
	}

	#[inline(always)]
	pub fn parse_fn<'b>(
		&'b self,
		tokens: &mut Peekable<Iter<'b, TokenInfo>>,
		i: String,
	) -> Result<(Expression, usize), Error> {
		let mut depth = 0;
		let mut params = vec![];
		let mut expression = vec![];

		let start = tokens.peek().unwrap().range.start;
		let mut end = tokens.next().unwrap().range.end;

		loop {
			let tokeninfo = tokens.next();

			if tokeninfo.is_none() {
				break;
			}

			let tokeninfo = &tokeninfo.unwrap();

			let token = &tokeninfo.token;

			end = tokeninfo.range.end;

			if *token == Token::RParen {
				if depth == 0 {
					if !expression.is_empty() && depth == 0 {
						let mut lex = expression.iter().peekable();
						let data = self.parser(&mut lex, 0)?.0;

						params.push((data, start..end));
						expression.clear();
					}
					break;
				}
				depth -= 1;
			}

			if *token == Token::LParen {
				depth += 1;
			}

			if *token == Token::Comma && depth == 0 {
				let mut lex = expression.iter().peekable();
				let data = self.parser(&mut lex, 0)?.0;

				params.push((data, start..end));

				expression.clear();
				continue;
			}

			expression.push((*tokeninfo).to_owned());
		}

		if !expression.is_empty() {
			let mut lex = expression.iter().peekable();
			let data = self.parser(&mut lex, 0)?.0;

			params.push((data, start..end));
			expression.clear();
		}

		Ok((Expression::FunctionCall(i.to_string(), params), end))
	}

	#[inline(always)]
	pub fn parse_if<'b>(
		&'b self,
		tokens: &mut Peekable<Iter<'b, TokenInfo>>,
	) -> Result<(Expression, usize), Error> {
		let mut depth = 1;
		let mut params = vec![];
		let mut expression = vec![];

		let mut end = tokens.peek().unwrap().range.end;

		loop {
			let tokeninfo = tokens.next();

			if tokeninfo.is_none() {
				break;
			}

			let tokeninfo = &tokeninfo.unwrap();

			let token = &tokeninfo.token;

			end = tokeninfo.range.end;

			if *token == Token::Then || *token == Token::Else {
				let mut lex = expression.iter().peekable();
				let data = self.parser(&mut lex, 0)?.0;

				params.push(data);
				expression.clear();
				continue;
			}

			if *token == Token::If {
				depth += 1;
			}

			if *token == Token::End {
				depth -= 1;
				if depth == 0 {
					break;
				}
			}

			expression.push((*tokeninfo).to_owned());
		}

		if !expression.is_empty() {
			let mut lex = expression.iter().peekable();
			let data = self.parser(&mut lex, 0)?.0;

			params.push(data);
			expression.clear();
		}

		Ok((
			Expression::Branched(
				Box::new(params[0].clone()),
				Box::new(params[1].clone()),
				Box::new(params[2].clone()),
			),
			end,
		))
	}

	fn infix_binding_power(&self, op: &Token) -> (u16, u16) {
		match op {
			Token::Add | Token::Sub => (1, 2),
			Token::Mul | Token::Div | Token::Rem => (3, 4),
			Token::Pow => (5, 6),
			Token::IsEq | Token::NEq | Token::Gt | Token::Lt | Token::GtEq | Token::LtEq => (7, 8),
			Token::If | Token::Then | Token::Else | Token::End => (9, 10),
			_ => (0, 0),
		}
	}
}
