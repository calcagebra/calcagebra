use std::{iter::Peekable, ops::RangeInclusive, slice::Iter};

use crate::{
	ast::{AstNode, Expression},
	errors::ErrorReporter,
	token::{Token, TokenInfo},
	types::NumberType,
};

pub struct Parser {
	file: String,
	tokens: Vec<Vec<TokenInfo>>,
	reporter: ErrorReporter,
}

impl Parser {
	pub fn new(file: &str, tokens: Vec<Vec<TokenInfo>>, reporter: ErrorReporter) -> Self {
		Self {
			file: file.to_string(),
			tokens,
			reporter,
		}
	}

	pub fn ast(&self) -> Vec<AstNode> {
		let mut ast = vec![];
		let lines = &self.tokens;

		for line in lines {
			let mut tokens = line.iter().peekable();

			let identifier = tokens.next().unwrap();

			match identifier.token {
				Token::Import => {
					if let Token::Identifier(ident) = &tokens.peek().unwrap().token {
						tokens.next();
						ast.push(AstNode::Import(ident.to_owned()));
						continue;
					}
					let tokeninfo = tokens.next().unwrap();

					self.reporter.syntax_error(
						&self.file,
						&tokeninfo.range,
						(&Token::Identifier("ident".to_string()), &tokeninfo.token),
					)
				}
				Token::Let => {
					let mut datatype = None;

					if tokens.peek().unwrap().token == Token::Colon {
						tokens.next();

						if let Token::Identifier(ident) = &tokens.peek().unwrap().token {
							tokens.next();

							datatype = Some(NumberType::parse(ident))
						} else {
							let tokeninfo = tokens.next().unwrap();

							self.reporter.syntax_error(
								&self.file,
								&tokeninfo.range,
								(&Token::Identifier("ident".to_string()), &tokeninfo.token),
							)
						}
					}

					let name = match &tokens.next().unwrap().token {
						Token::Identifier(name) => name,
						_ => unreachable!(),
					};

					tokens.next(); // Token =

					let (expr, _, range) = self.pratt_parser(tokens, 0);

					let expr_type = expr.infer_datatype();

					if datatype.is_none() {
						datatype = expr_type
					}

					if expr_type.is_some() && expr_type.unwrap() != datatype.unwrap() {
						self
							.reporter
							.type_error(&self.file, &range, (datatype.unwrap(), expr_type.unwrap()));
					}

					ast.push(AstNode::Assignment(
						(name.to_string(), datatype.unwrap()),
						expr,
					));
				}
				Token::Fn => {
					let name = match &tokens.next().unwrap().token {
						Token::Identifier(name) => name,
						_ => unreachable!(),
					};

					tokens.next(); // Token (

					let mut args = vec![];

					loop {
						let t = tokens.peek();

						if t.is_none() {
							break;
						}

						if Token::RParen == t.unwrap().token {
							tokens.next();
							break;
						}

						let t = tokens.next().unwrap();

						let mut datatype = Some(NumberType::Real);

						if tokens.peek().unwrap().token == Token::Colon {
							tokens.next();

							if let Token::Identifier(ident) = &tokens.peek().unwrap().token {
								tokens.next();

								datatype = Some(NumberType::parse(ident))
							} else {
								let tokeninfo = tokens.next().unwrap();

								self.reporter.syntax_error(
									&self.file,
									&tokeninfo.range,
									(&Token::Identifier("ident".to_string()), &tokeninfo.token),
								)
							}
						}

						args.push((
							match &t.token {
								Token::Identifier(i) => i.to_string(),
								_ => unreachable!(),
							},
							datatype.unwrap(),
						));
					}

					let mut return_type = Some(NumberType::Real);

					if tokens.peek().unwrap().token == Token::Colon {
						tokens.next();

						if let Token::Identifier(ident) = &tokens.peek().unwrap().token {
							tokens.next();

							return_type = Some(NumberType::parse(ident))
						} else {
							let tokeninfo = tokens.next().unwrap();

							self.reporter.syntax_error(
								&self.file,
								&tokeninfo.range,
								(&Token::Identifier("ident".to_string()), &tokeninfo.token),
							)
						}
					}

					let tokeninfo = tokens.next().unwrap();

					if tokeninfo.token != Token::Eq {
						self.reporter.syntax_error(
							&self.file,
							&tokeninfo.range,
							(&Token::LParen, &tokeninfo.token),
						);
					}

					let (expr, _, range) = self.pratt_parser(tokens, 0);

					let expr_type = expr.infer_datatype();

					if expr_type.is_some() && expr_type.unwrap() != return_type.unwrap() {
						self.reporter.type_error(
							&self.file,
							&range,
							(return_type.unwrap(), expr_type.unwrap()),
						);
					}

					ast.push(AstNode::FunctionDeclaration(
						name.to_string(),
						args,
						return_type.unwrap(),
						expr,
					));
				}
				_ => {
					let args = self.pratt_parser(line.iter().peekable(), 0).0;

					match args {
						Expression::FunctionCall(name, args) => {
							ast.push(AstNode::FunctionCall(name.to_string(), args))
						}
						_ => unreachable!(),
					}
				}
			}
		}
		ast
	}

	pub fn pratt_parser<'a>(
		&'a self,
		mut tokens: Peekable<Iter<'a, TokenInfo>>,
		prec: u16,
	) -> (
		Expression,
		Peekable<Iter<'a, TokenInfo>>,
		RangeInclusive<usize>,
	) {
		let tokeninfo = &tokens.next().unwrap();

		let token = &tokeninfo.token;
		let mut expr: Option<Expression> = None;

		let start = *tokeninfo.range.start();
		let mut end;

		match token {
			Token::Identifier(i) => {
				// An identifier can either be a function call, in multiplication with a mod
				// or simply an identifier, eg read(), a|b|, c

				if tokens.peek().is_some()
					&& self.infix_binding_power(&tokens.peek().unwrap().token) == (0, 0)
					&& ![Token::RParen, Token::Abs].contains(&tokens.peek().unwrap().token)
				{
					(expr, tokens, end) = self.parse_fn(tokens, i.clone());
				} else {
					end = *tokeninfo.range.end();
					expr = Some(Expression::Identifier(i.to_string()));

					if tokens.peek().is_some()
						&& tokens.peek().unwrap().token == Token::Identifier("i".to_owned())
					{
						expr = Some(Expression::Complex(
							Box::new(Expression::Real(0.0)),
							Box::new(Expression::Identifier(i.to_string())),
						));
						end = *tokens.next().unwrap().range.end();
					}
				};
			}
			Token::LParen => {
				let exp;
				let range;

				(exp, tokens, range) = self.pratt_parser(tokens, 0);

				end = *range.end();
				expr = Some(exp);
				tokens.next();
			}
			Token::Abs => {
				let exp;
				let range;

				(exp, tokens, range) = self.pratt_parser(tokens, 0);

				end = *range.end();
				expr = Some(Expression::Abs(Box::new(exp)));
				tokens.next();
			}
			Token::If => {
				(expr, tokens, end) = self.parse_if(tokens);
			}
			Token::Sub => {
				if let Token::Integer(i) = tokens.peek().unwrap().token {
					expr = Some(Expression::Integer(-i));
					end = *tokens.next().unwrap().range.end();
				} else if let Token::Float(i) = tokens.peek().unwrap().token {
					expr = Some(Expression::Real(-i));
					end = *tokens.next().unwrap().range.end();
				} else {
					end = *tokeninfo.range.end();
				}
			}
			Token::Integer(n) => {
				expr = Some(Expression::Integer(*n));
				end = *tokeninfo.range.end();

				if tokens.peek().is_some()
					&& tokens.peek().unwrap().token == Token::Identifier("i".to_owned())
				{
					expr = Some(Expression::Complex(
						Box::new(Expression::Real(0.0)),
						Box::new(Expression::Real(*n as f32)),
					));
					end = *tokens.next().unwrap().range.end();
				}
			}
			Token::Float(n) => {
				expr = Some(Expression::Real(*n));
				end = *tokeninfo.range.end();

				if tokens.peek().is_some()
					&& tokens.peek().unwrap().token == Token::Identifier("i".to_owned())
				{
					expr = Some(Expression::Complex(
						Box::new(Expression::Real(0.0)),
						Box::new(Expression::Real(*n)),
					));
					end = *tokens.next().unwrap().range.end();
				}
			}
			_ => {
				end = *tokeninfo.range.end();
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

			(rhs, tokens, range) = self.pratt_parser(tokens, rbp);

			end = *range.end();
			expr = Some(Expression::Binary(
				Box::new(expr.unwrap()),
				op.token.clone(),
				Box::new(rhs),
			));
		}

		(expr.unwrap(), tokens, start..=end)
	}

	pub fn parse_fn<'a>(
		&'a self,
		mut tokens: Peekable<Iter<'a, TokenInfo>>,
		i: String,
	) -> (Option<Expression>, Peekable<Iter<'a, TokenInfo>>, usize) {
		let mut depth = 0;
		let mut params = vec![];
		let mut expression = vec![];

		let mut end = *tokens.next().unwrap().range.end();

		loop {
			let tokeninfo = tokens.next();

			if tokeninfo.is_none() {
				break;
			}

			let tokeninfo = &tokeninfo.unwrap();

			let token = &tokeninfo.token;

			end = *tokeninfo.range.end();

			if *token == Token::RParen {
				if depth == 0 {
					if !expression.is_empty() && depth == 0 {
						let lex = expression.iter().peekable();
						let data = self.pratt_parser(lex, 0).0;

						params.push(data);
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
				let lex = expression.iter().peekable();
				let data = self.pratt_parser(lex, 0).0;

				params.push(data);

				expression.clear();
				continue;
			}

			expression.push((*tokeninfo).to_owned());
		}
		if !expression.is_empty() {
			let lex = expression.iter().peekable();
			let data = self.pratt_parser(lex, 0).0;

			params.push(data);
			expression.clear();
		}

		(
			Some(Expression::FunctionCall(i.to_string(), params)),
			tokens,
			end,
		)
	}

	pub fn parse_if<'a>(
		&'a self,
		mut tokens: Peekable<Iter<'a, TokenInfo>>,
	) -> (Option<Expression>, Peekable<Iter<'a, TokenInfo>>, usize) {
		let mut depth = 1;
		let mut params = vec![];
		let mut expression = vec![];

		let mut end = *tokens.peek().unwrap().range.end();

		loop {
			let tokeninfo = tokens.next();

			if tokeninfo.is_none() {
				break;
			}

			let tokeninfo = &tokeninfo.unwrap();

			let token = &tokeninfo.token;

			end = *tokeninfo.range.end();

			if *token == Token::Then || *token == Token::Else {
				let lex = expression.iter().peekable();
				let data = self.pratt_parser(lex, 0).0;

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
			let lex = expression.iter().peekable();
			let data = self.pratt_parser(lex, 0).0;

			params.push(data);
			expression.clear();
		}

		(
			Some(Expression::Branched(
				Box::new(params[0].clone()),
				Box::new(params[1].clone()),
				Box::new(params[2].clone()),
			)),
			tokens,
			end,
		)
	}

	fn infix_binding_power(&self, op: &Token) -> (u16, u16) {
		match op {
			Token::Add | Token::Sub => (1, 2),
			Token::Mul | Token::Div | Token::Rem => (3, 4),
			Token::Pow => (5, 6),
			Token::IsEq
			| Token::NEq
			| Token::Gt
			| Token::Lt
			| Token::GtEq
			| Token::LtEq
			| Token::Belongs => (7, 8),
			Token::If | Token::Then | Token::Else | Token::End => (9, 10),
			_ => (0, 0),
		}
	}
}
