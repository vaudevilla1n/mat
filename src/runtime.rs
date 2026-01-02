use std::{ fmt, error };
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

#[allow(unused_variables)]
fn eval_expression(e: Expr) -> Result<Object, Error> {
	match e {
		Expr::Binary(left, op, right) => {
			let x = eval_expression(*left)?;
			let y = eval_expression(*right)?;

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
			let x = eval_expression(*right)?;

			match (op, x) {
				(Token::Minus, Object::Number(x)) => Ok(Object::Number(-x)),
				(Token::Plus, Object::Number(x)) => Ok(Object::Number(x)),

				_ => Err(Error::new("invalid unary operands")),
			}
		}

		Expr::Group(expr) => eval_expression(*expr),

		Expr::Call(func_name, args) => { todo!() }

		Expr::Number(x) => Ok(Object::Number(x)),

		Expr::Bool(b) => Ok(Object::Bool(b)),

		Expr::Var(var_name) => { todo!() }
	}
}

#[allow(unused_variables)]
pub fn eval(stmt: Stmt) -> Result<(), Error> {
	match stmt {
		// Body(Vec<Stmt>) => {}

		Stmt::If(cond, body, else_body) => { todo!(); }

		Stmt::Func(func_name, args, body) => { todo!(); }

		Stmt::Assign(var_name, expr) => { todo!(); }

		Stmt::Print(expr) => {
			let obj = eval_expression(expr)?; 
			println!("{obj}");

			Ok(())
		}

		Stmt::Return(expr) => { todo!(); }

		_ => { unreachable!(); }
	}
}
