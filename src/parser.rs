use std::{iter::Peekable, slice::Iter};

use crate::{
	ast::{AstNode, AstType, Expression},
	errors::ErrorReporter,
	token::{Token, TokenInfo},
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

			let datatype = if tokens.peek().unwrap().token == Token::Colon {
				tokens.next();

				if let Token::Identifier(ident) = &tokens.peek().unwrap().token {
					tokens.next();
					AstType::parse(ident)
				} else {
					panic!("expected identifier token got {:?}", tokens.next().unwrap());
				}
			} else {
				AstType::Float
			};

			if tokens.peek().unwrap().token == Token::Eq {
				tokens.next();
				let mut name = "";
				if let Token::Identifier(str) = &identifier.token {
					name = str;
				}

				let expr = self.pratt_parser(tokens, 0).0;

				let expr_type = expr.infer_datatype();

				if expr_type.is_some() && expr_type.unwrap() != datatype {
					self
						.reporter
						.type_error(&self.file, 1..=2, (datatype, expr_type.unwrap()));
				}

				ast.push(AstNode::Assignment((name.to_string(), datatype), expr));
			} else {
				let name = match &identifier.token {
					Token::Identifier(name) => name,
					_ => unreachable!(),
				};

				if line
					.iter()
					.map(|f| &f.token)
					.collect::<Vec<&Token>>()
					.contains(&&Token::Eq)
				{
					assert!(tokens.next().unwrap().token == Token::LParen);

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

						let datatype = if tokens.peek().unwrap().token == Token::Colon {
							tokens.next();

							if let Token::Identifier(ident) = &tokens.peek().unwrap().token {
								tokens.next();
								AstType::parse(ident)
							} else {
								panic!("expected identifier token got {:?}", tokens.next().unwrap());
							}
						} else {
							AstType::Float
						};

						args.push((
							match &t.token {
								Token::Identifier(i) => i.to_string(),
								_ => unreachable!(),
							},
							datatype,
						));
					}

					let return_type = if tokens.peek().unwrap().token == Token::Colon {
						tokens.next();

						if let Token::Identifier(ident) = &tokens.peek().unwrap().token {
							tokens.next();
							AstType::parse(ident)
						} else {
							panic!("expected identifier token got {:?}", tokens.next().unwrap());
						}
					} else {
						AstType::Float
					};

					assert!(tokens.next().unwrap().token == Token::Eq);

					let expr = self.pratt_parser(tokens, 0).0;
					ast.push(AstNode::FunctionDeclaration(
						name.to_string(),
						args,
						return_type,
						expr,
					));
				} else {
					let (args, _) = self.pratt_parser(line.iter().peekable(), 0);

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
	) -> (Expression, Peekable<Iter<TokenInfo>>) {
		let token = &tokens.next().unwrap().token;
		let mut expr: Option<Expression> = None;
		match token {
			Token::Identifier(i) => {
				if tokens.peek().is_some()
					&& self.infix_binding_power(&tokens.peek().unwrap().token) == (0, 0)
					&& ![Token::RParen, Token::Abs].contains(&tokens.peek().unwrap().token)
				{
					(expr, tokens) = self.parse_fn(tokens, i.clone());
				} else {
					expr = Some(Expression::Identifier(i.to_string()))
				};
			}
			Token::LParen => {
				let exp;
				(exp, tokens) = self.pratt_parser(tokens, 0);
				expr = Some(exp);
				tokens.next();
			}
			Token::Abs => {
				let exp;
				(exp, tokens) = self.pratt_parser(tokens, 0);
				expr = Some(Expression::Abs(Box::new(exp)));
				tokens.next();
			}
			Token::If => {
				(expr, tokens) = self.parse_if(tokens);
			}
			Token::Sub => {
				if let Token::Integer(i) = tokens.peek().unwrap().token {
					expr = Some(Expression::Integer(-i));
					tokens.next();
				}
				if let Token::Float(i) = tokens.peek().unwrap().token {
					expr = Some(Expression::Float(-i));
					tokens.next();
				}
			}
			Token::Integer(n) => expr = Some(Expression::Integer(*n)),
			Token::Float(n) => expr = Some(Expression::Float(*n)),
			_ => {}
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
			(rhs, tokens) = self.pratt_parser(tokens, rbp);
			expr = Some(Expression::Binary(
				Box::new(expr.unwrap()),
				op.token.clone(),
				Box::new(rhs),
			));
		}

		(expr.unwrap(), tokens)
	}

	pub fn parse_fn<'a>(
		&'a self,
		mut tokens: Peekable<Iter<'a, TokenInfo>>,
		i: String,
	) -> (Option<Expression>, Peekable<Iter<'a, TokenInfo>>) {
		let mut depth = 0;
		let mut params = vec![];
		let mut expression = vec![];

		tokens.next();

		loop {
			let tokeninfo = tokens.next();

			if tokeninfo.is_none() {
				break;
			}

			let token = &tokeninfo.unwrap().token.clone();

			if *token == Token::RParen {
				if depth == 0 {
					if !expression.is_empty() && depth == 0 {
						let lex = expression.iter().peekable();
						let (data, _) = self.pratt_parser(lex, 0);

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
				let (data, _) = self.pratt_parser(lex, 0);

				params.push(data);

				expression.clear();
				continue;
			}

			expression.push(tokeninfo.unwrap().to_owned());
		}
		if !expression.is_empty() {
			let lex = expression.iter().peekable();
			let (data, _) = self.pratt_parser(lex, 0);

			params.push(data);
			expression.clear();
		}

		(
			Some(Expression::FunctionCall(i.to_string(), params)),
			tokens,
		)
	}

	pub fn parse_if<'a>(
		&'a self,
		mut tokens: Peekable<Iter<'a, TokenInfo>>,
	) -> (Option<Expression>, Peekable<Iter<'a, TokenInfo>>) {
		let mut depth = 1;
		let mut params = vec![];
		let mut expression = vec![];

		loop {
			let tokeninfo = tokens.next();

			if tokeninfo.is_none() {
				break;
			}

			let token = &tokeninfo.unwrap().token;

			if *token == Token::Then || *token == Token::Else {
				let lex = expression.iter().peekable();
				let (data, _) = self.pratt_parser(lex, 0);

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

			expression.push(tokeninfo.unwrap().to_owned());
		}

		if !expression.is_empty() {
			let lex = expression.iter().peekable();
			let (data, _) = self.pratt_parser(lex, 0);

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
