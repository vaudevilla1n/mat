use std::{ rc::Rc, fmt, error };
use crate::lexer::*;


#[derive(Debug)]
pub enum Expr<'s> {
	Binary(Rc<Expr<'s>>, Token, Rc<Expr<'s>>),
	Unary(Token, Rc<Expr<'s>>),
	Group(Rc<Expr<'s>>),
	Call(String, &'s[Rc<Expr<'s>>]),
	Number(f64),
	Var(String),
}

#[derive(Debug)]
pub enum Stmt<'s> {
	If(Expr<'s>, &'s[Rc<Stmt<'s>>], &'s[Rc<Stmt<'s>>]),
	Func(String, &'s[Rc<Stmt<'s>>]),
	Assign(String, Expr<'s>),
	Expression(Expr<'s>),
}

#[derive(Debug)]
pub struct Error {
	col: usize,
	line: usize,
	msg: String,
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "parser error: line {}, col {}: {}", self.line, self.col, self.msg)
	}
}

impl error::Error for Error { }

#[derive(Debug)]
pub struct Parser<'s> {
	tokens: Lexer<'s>,
}

#[allow(dead_code)]
impl<'s> Parser<'s> {
	pub fn new(l: Lexer<'s>) -> Parser<'s> {
		Parser{tokens: l}
	}

	fn err(&self, msg: String) -> Error {
		let (line, col) = self.tokens.pos();

		Error{line: line, col: col, msg: msg}
	}

	fn expect(&mut self, expected: Token, msg: String) -> Result<(), Error> {
		if self.tokens.peek() == expected {
			self.tokens.next();

			Ok(())
		} else {
			Err(self.err(msg))
		}
	}

	fn call(&mut self) -> Result<Expr<'s>, Error> {
		todo!();
	}

	fn literal(&mut self) -> Result<Expr<'s>, Error> {
		match self.tokens.next() {
			Token::Number(x) => Ok(Expr::Number(x)),

			Token::LParen => {
				let e = self.expr()?;

				self.expect(Token::RParen, "expected closing parentheses".to_string())?;

				Ok(Expr::Group(Rc::new(e)))
			}

			Token::Identifier(name) => {
				if self.tokens.peek() == Token::LParen {
					self.call()
				} else {
					Ok(Expr::Var(name))
				}
			}

			tok => Err(self.err(format!("expected literal got {tok:?}"))),
		}
	}

	fn unary(&mut self) -> Result<Expr<'s>, Error> {
		if let Token::Plus | Token::Minus = self.tokens.peek() {
			let op = self.tokens.next();
			let right = self.literal()?;

			Ok(Expr::Unary(op, Rc::new(right)))
		} else {
			self.literal()
		}
	}

	fn exponent(&mut self) -> Result<Expr<'s>, Error> {
		let left = self.unary()?;

		if let Token::Caret = self.tokens.peek() {
			let op = self.tokens.next();
			let right = self.unary()?;

			Ok(Expr::Binary(Rc::new(left), op, Rc::new(right)))
		} else {
			Ok(left)
		}
	}

	fn factor(&mut self) -> Result<Expr<'s>, Error> {
		let mut e = self.exponent()?;

		while let Token::Star | Token::Slash = self.tokens.peek() {
			let op = self.tokens.next();
			let right = self.exponent()?;

			e = Expr::Binary(Rc::new(e), op, Rc::new(right));
		}

		Ok(e)
	}

	fn term(&mut self) -> Result<Expr<'s>, Error> {
		let mut e = self.factor()?;

		while let Token::Plus | Token::Minus = self.tokens.peek() {
			let op = self.tokens.next();
			let right = self.factor()?;

			e = Expr::Binary(Rc::new(e), op, Rc::new(right));
		}

		Ok(e)
	}

	fn comparison(&mut self) -> Result<Expr<'s>, Error> {
		let mut e = self.term()?;

		while let Token::Equal | Token::Greater | Token::GreaterEqual | Token::Less | Token::LessEqual = self.tokens.peek() {
			let op = self.tokens.next();
			let right = self.term()?;

			e = Expr::Binary(Rc::new(e), op, Rc::new(right));
		}

		Ok(e)
	}

	fn expr(&mut self) -> Result<Expr<'s>, Error> {
		self.comparison()
	}

	fn expression(&mut self) -> Result<Stmt<'s>, Error> {
		let expr = self.expr()?;

		self.expect(Token::Semicolon, "missing semicolon".to_string())?;

		Ok(Stmt::Expression(expr))
	}

	fn assign(&mut self) -> Result<Stmt<'s>, Error> {
		todo!();
	}

	pub fn parse(&mut self) -> Result<Vec<Stmt<'s>>, Error> {
		let mut stmts: Vec<Stmt<'s>> = Vec::new();

		while self.tokens.peek() != Token::EOF {
			let stmt = match self.tokens.peek() {
				Token::Number(_) | Token::LParen => {
					self.expression()?
				}

				Token::Identifier(_) => {
					let mut tokens = self.tokens.clone();
					tokens.next();

					if tokens.peek() == Token::Assign {
						self.assign()?
					} else {
						self.expression()?
					}
				}

				tok => {
					return Err(self.err(format!("unexpected token {tok:?}")))
				}
			};

			stmts.push(stmt);
		}

		Ok(stmts)
	}
}
