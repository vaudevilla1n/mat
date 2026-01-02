use std::{ fmt, error, collections::HashMap };
use crate::parser::{Expr, Stmt};
use crate::lexer::Token;

#[derive(Debug)]
pub struct Error {
	msg: String,
}

impl Error {
	pub fn new(msg: &str) -> Error {
		Error{msg: msg.to_string()}
	}
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "runtime error: {}", self.msg)
	}
}

impl error::Error for Error {}

#[derive(Clone)]
#[derive(Debug)]
pub enum Object {
	Number(f64),
	Bool(bool),
}

impl fmt::Display for Object {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match *self {
			Object::Bool(v) => write!(f, "{}", v),
			Object::Number(v) => write!(f, "{}", v),
		}
	}
}

pub struct Runtime {
	variable_scope: Vec<HashMap<String, Object>>,
	scope_depth: usize,
	ret: Option<Object>,
}

impl Runtime {
	pub fn new() -> Runtime {
		Runtime{
			variable_scope: vec![HashMap::new()],
			scope_depth: 0, // Global scope is 0, obviously
			ret: None,
		}
	}

	fn set_variable(&mut self, var_name: String, val: Object) {
		let scope = &mut self.variable_scope[self.scope_depth];
		scope.insert(var_name, val);
	}

	fn get_variable(&self, var_name: &String) -> Option<Object> {
		// try to find variable in the current function's scope then in the global scope

		let scope = &self.variable_scope[self.scope_depth];

		if let Some(o) = scope.get(var_name) {
			return Some(o.clone());
		}

		if self.scope_depth != 0 {
			let global_scope = &self.variable_scope[0];

			match global_scope.get(var_name) {
				Some(o) => Some(o.clone()),
				None => None,
			}
		} else {
			None
		}
	}

	#[allow(unused_variables)]
	fn expression(&self, e: Expr) -> Result<Object, Error> {
		match e {
			Expr::Binary(left, op, right) => {
				let x = self.expression(*left)?;
				let y = self.expression(*right)?;

				match (x, op, y) {
					(Object::Number(x), Token::Plus, Object::Number(y)) => Ok(Object::Number(x + y)),
					(Object::Number(x), Token::Minus, Object::Number(y)) => Ok(Object::Number(x - y)),
					(Object::Number(x), Token::Star, Object::Number(y)) => Ok(Object::Number(x * y)),
					(Object::Number(x), Token::Slash, Object::Number(y)) => {
						if y == 0.0 {
							Err(Error::new("division by zero"))
						} else {
							Ok(Object::Number(x / y))
						}
					}
					(Object::Number(x), Token::Caret, Object::Number(y)) => Ok(Object::Number(x.powf(y))),

					(Object::Number(x), Token::Less, Object::Number(y)) => Ok(Object::Bool(x < y)),
					(Object::Number(x), Token::LessEqual, Object::Number(y)) => Ok(Object::Bool(x <= y)),
					(Object::Number(x), Token::Equal, Object::Number(y)) => Ok(Object::Bool(x == y)),
					(Object::Bool(x), Token::Equal, Object::Bool(y)) => Ok(Object::Bool(x == y)),
					(Object::Number(x), Token::Greater, Object::Number(y)) => Ok(Object::Bool(x > y)),
					(Object::Number(x), Token::GreaterEqual, Object::Number(y)) => Ok(Object::Bool(x >= y)),

					_ => Err(Error::new("invalid binary operands")),
				}
			}

			Expr::Unary(op, right) => {
				let x = self.expression(*right)?;

				match (op, x) {
					(Token::Minus, Object::Number(x)) => Ok(Object::Number(-x)),
					(Token::Plus, Object::Number(x)) => Ok(Object::Number(x)),

					_ => Err(Error::new("invalid unary operands")),
				}
			}

			Expr::Group(expr) => self.expression(*expr),

			Expr::Call(func_name, args) => { todo!() }

			Expr::Number(x) => Ok(Object::Number(x)),

			Expr::Bool(b) => Ok(Object::Bool(b)),

			Expr::Var(var_name) => {
				match self.get_variable(&var_name) {
					Some(o) => Ok(o.clone()),
					None => Err(Error::new(&format!("variable '{var_name}' not found"))),
				}
			}
		}
	}

	#[allow(unused_variables)]
	fn stmt(&mut self, stmt: Stmt) -> Result<(), Error> {
		match stmt {
			Stmt::Body(_) => { self.body(stmt)?; }

			Stmt::If(cond, body, else_body) => { self.if_stmt(cond, body, else_body)?; }

			Stmt::Func(func_name, args, body) => { todo!(); }

			Stmt::Assign(var_name, expr) => {
				let val = self.expression(expr)?;
				self.set_variable(var_name.to_string(), val)
			}

			Stmt::Print(expr) => {
				let obj = self.expression(expr)?; 
				println!("{obj}");
			}

			Stmt::Return(expr) => {
				let obj = self.expression(expr)?; 
				self.ret = Some(obj);
			}
		}

		Ok(())
	}

	fn body(&mut self, body: Stmt) -> Result<(), Error> {
		let body = match body {
			Stmt::Body(body) => body,
			_ => unreachable!(),
		};

		for stmt in body {
			self.stmt(stmt)?;
		}

		Ok(())
	}

	fn if_stmt(&mut self, cond: Expr, body: Box<Stmt>, else_body: Option<Box<Stmt>>) -> Result<(), Error> {
		let cond = self.expression(cond)?;
		match cond {
			Object::Bool(true) => self.body(*body),

			Object::Bool(false) => {
				if let Some(else_body) = else_body {
					match *else_body {
						// either an if statement or a regular body
						Stmt::If(cond, body, else_body) => self.if_stmt(cond, body, else_body),

						body => self.body(body),
					}
				} else {
					Ok(())
				}
			}

			_ => Err(Error::new("invalid expression for if statement")),
		}
	}

	#[allow(unused_variables)]
	pub fn eval(&mut self, stmts: Vec<Stmt>) -> Result<(), Error> {
		for stmt in stmts {
			println!("{stmt:?}");

			match stmt {
				// Body(Vec<Stmt>) => {}

				Stmt::If(cond, body, else_body) => { self.if_stmt(cond, body, else_body)?; }

				Stmt::Func(func_name, args, body) => { todo!(); }

				Stmt::Assign(var_name, expr) => {
					let val = self.expression(expr)?;
					self.set_variable(var_name, val);
				}

				Stmt::Print(expr) => {
					let obj = self.expression(expr)?; 
					println!("{obj}");
				}

				_ => { unreachable!(); }
			}
		}

		Ok(())
	}
}

