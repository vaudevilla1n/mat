use std::{ rc::Rc, fmt, error };
use crate::lexer::*;


#[derive(Debug)]
pub enum Expr {
	Binary(Rc<Expr>, Token, Rc<Expr>),
	Unary(Token, Rc<Expr>),
	Group(Rc<Expr>),
	Call(String, Vec<Expr>),
	Number(f64),
	Bool(bool),
	Var(String),
}

#[derive(Debug)]
pub enum Stmt {
	Body(Vec<Stmt>),
	If(Expr, Rc<Stmt>, Option<Rc<Stmt>>),
	Func(String, Vec<String>, Rc<Stmt>),
	Assign(String, Expr),
	Print(Expr),
	Return(Expr),
	Expression(Expr),
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

	fn err(&self, msg: &str) -> Error {
		let (line, col) = self.tokens.pos();

		Error{line: line, col: col, msg: msg.to_string()}
	}

	fn check(&mut self, expected: Token) -> bool {
		if self.tokens.peek() == expected {
			true
		} else {
			false
		}
	}

	fn consumed(&mut self, expected: Token) -> bool {
		if self.check(expected) {
			self.tokens.next();

			true
		} else {
			false
		}
	}

	fn expect(&mut self, expected: Token, msg: &str) -> Result<(), Error> {
		if self.consumed(expected) {
			Ok(())
		} else {
			Err(self.err(msg))
		}
	}

	fn call(&mut self, func_name: String) -> Result<Expr, Error> {
		let mut args: Vec<Expr> = Vec::new();
		if self.tokens.peek() != Token::RParen {
			let arg = self.expr()?;
			args.push(arg);

			while self.consumed(Token::Comma) {
				let arg = self.expr()?;
				args.push(arg);
			}

		}

		self.expect(Token::RParen, "missing closing parentheses")?;

		Ok(Expr::Call(func_name, args))
	}

	fn literal(&mut self) -> Result<Expr, Error> {
		match self.tokens.next() {
			Token::Number(x) => Ok(Expr::Number(x)),

			Token::True => Ok(Expr::Bool(true)),
			Token::False => Ok(Expr::Bool(false)),

			Token::LParen => {
				let e = self.expr()?;

				self.expect(Token::RParen, "expected closing parentheses")?;

				Ok(Expr::Group(Rc::new(e)))
			}

			Token::Identifier(name) => {
				if self.consumed(Token::LParen) {
					self.call(name)
				} else {
					Ok(Expr::Var(name))
				}
			}

			tok => Err(self.err(&format!("expected literal got {tok:?}"))),
		}
	}

	fn unary(&mut self) -> Result<Expr, Error> {
		if let Token::Plus | Token::Minus = self.tokens.peek() {
			let op = self.tokens.next();
			let right = self.literal()?;

			Ok(Expr::Unary(op, Rc::new(right)))
		} else {
			self.literal()
		}
	}

	fn exponent(&mut self) -> Result<Expr, Error> {
		let left = self.unary()?;

		if let Token::Caret = self.tokens.peek() {
			let op = self.tokens.next();
			let right = self.unary()?;

			Ok(Expr::Binary(Rc::new(left), op, Rc::new(right)))
		} else {
			Ok(left)
		}
	}

	fn factor(&mut self) -> Result<Expr, Error> {
		let mut e = self.exponent()?;

		while let Token::Star | Token::Slash = self.tokens.peek() {
			let op = self.tokens.next();
			let right = self.exponent()?;

			e = Expr::Binary(Rc::new(e), op, Rc::new(right));
		}

		Ok(e)
	}

	fn term(&mut self) -> Result<Expr, Error> {
		let mut e = self.factor()?;

		while let Token::Plus | Token::Minus = self.tokens.peek() {
			let op = self.tokens.next();
			let right = self.factor()?;

			e = Expr::Binary(Rc::new(e), op, Rc::new(right));
		}

		Ok(e)
	}

	fn comparison(&mut self) -> Result<Expr, Error> {
		let mut e = self.term()?;

		while let Token::Equal | Token::Greater | Token::GreaterEqual | Token::Less | Token::LessEqual = self.tokens.peek() {
			let op = self.tokens.next();
			let right = self.term()?;

			e = Expr::Binary(Rc::new(e), op, Rc::new(right));
		}

		Ok(e)
	}

	fn expr(&mut self) -> Result<Expr, Error> {
		self.comparison()
	}

	fn expression(&mut self) -> Result<Stmt, Error> {
		let expr = self.expr()?;

		self.expect(Token::Semicolon, "missing semicolon")?;

		Ok(Stmt::Expression(expr))
	}

	fn assign(&mut self, var_name: String) -> Result<Stmt, Error> {
		let expr = self.expr()?;

		self.expect(Token::Semicolon, "missing semicolon")?;

		Ok(Stmt::Assign(var_name, expr))
	}

	fn return_stmt(&mut self) -> Result<Stmt, Error> {
		let expr = self.expr()?;
		self.expect(Token::Semicolon, "missing semicolon")?;

		Ok(Stmt::Return(expr))
	}

	fn print_stmt(&mut self) -> Result<Stmt, Error> {
		let expr = self.expr()?;
		self.expect(Token::Semicolon, "missing semicolon")?;

		Ok(Stmt::Print(expr))
	}

	fn body(&mut self) -> Result<Stmt, Error> {
		self.expect(Token::LBracket, "missing opening brace for if statement")?;

		let mut body: Vec<Stmt> = Vec::new();
		while !self.check(Token::RBracket) && !self.check(Token::EOF) {
			let stmt = self.statement()?;
			match stmt {
				Stmt::Func(_, _, _) => { return Err(self.err("function definitions musn't be nested")); }

				_ => { body.push(stmt); }
			};
		}

		self.expect(Token::RBracket, "missing closing brace")?;

		Ok(Stmt::Body(body))
	}

	fn if_stmt(&mut self) -> Result<Stmt, Error> {
		let condition = self.expr()?;
		let body = self.body()?;
		let else_body = if self.consumed(Token::Else) {
			if self.consumed(Token::If) {
				Some(Rc::new(self.if_stmt()?))
			} else {
				Some(Rc::new(self.body()?))
			}
		} else {
			None
		};

		Ok(Stmt::If(condition, Rc::new(body), else_body))
	}

	fn func(&mut self) -> Result<Stmt, Error> {
		let func_name = match self.tokens.next() {
			Token::Identifier(name) => name,

			_ => { return Err(self.err("need function name in declaration")) }
		};

		self.expect(Token::LParen, "expected opening parentheses after function name")?;

		let mut args: Vec<String> = Vec::new();
		if let Token::Identifier(arg) = self.tokens.peek() {
			self.tokens.next();
			args.push(arg);

			while self.consumed(Token::Comma) {
				let arg = match self.tokens.next() {
					Token::Identifier(var) => var,

					_ => { return Err(self.err("expected variable in function definition")); }
				};

				args.push(arg);
			}
		}

		self.expect(Token::RParen, "expected closing parentheses")?;

		let body = self.body()?;

		Ok(Stmt::Func(func_name, args, Rc::new(body)))
	}

	fn statement(&mut self) -> Result<Stmt, Error> {
		match self.tokens.peek() {
			Token::Number(_) | Token::LParen => {
				self.expression()
			}

			Token::Identifier(name) => {
				self.tokens.next();

				if self.consumed(Token::Assign) {
					self.assign(name)
				} else {
					self.expression()
				}
			}

			Token::If => {
				self.tokens.next();
				self.if_stmt()
			}

			Token::Return => {
				self.tokens.next();
				self.return_stmt()
			}

			Token::Print => {
				self.tokens.next();
				self.print_stmt()
			}

			Token::Func => {
				self.tokens.next();

				self.func()
			}

			tok => {
				Err(self.err(&format!("unexpected token {tok:?}")))
			}
		}
	}

	pub fn parse(&mut self) -> Result<Vec<Stmt>, Error> {
		let mut stmts: Vec<Stmt> = Vec::new();

		while self.tokens.peek() != Token::EOF {
			let stmt = self.statement()?;
			stmts.push(stmt);
		}

		Ok(stmts)
	}
}
