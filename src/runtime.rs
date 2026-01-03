use std::{ fmt, error, collections::HashMap, iter};
use crate::parser::{Expr, Stmt};
use crate::lexer::Token;

#[derive(Debug)]
pub struct Error {
	msg: String,
}

impl Error {
	pub fn new(msg: String) -> Error {
		Error{msg: msg}
	}
}

impl From<&str> for Error {
	fn from(msg: &str) -> Error {
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

type VariableScope = HashMap<String, Object>;
type FuncTable = HashMap<String, Func>;

#[derive(Clone)]
struct Func {
	args: Vec<String>,
	body: Vec<Stmt>,
}

pub struct Runtime {
	func_table: FuncTable,
	global_scope: VariableScope,
}

impl Runtime {
	pub fn new() -> Runtime {
		Runtime{
			func_table: HashMap::new(),
			global_scope: HashMap::new(),
		}
	}

	fn call(&mut self, local_scope: &mut Option<&mut VariableScope>, func_name: String, input: Vec<Expr>) -> Result<Object, Error> {
		let func = match self.func_table.get(&func_name) {
			Some(func) => func.clone(),
			None => { return Err(Error::new(format!("function '{func_name}' doesn't exist"))); }
		};

		let mut func_scope: VariableScope = HashMap::new();

		for (arg, input) in iter::zip(func.args, input) {
			let val = self.expression(local_scope, input)?;
			func_scope.insert(arg, val);
		}

		let ret = self.body(&mut Some(&mut func_scope), func.body)?;
		match ret {
			Some(obj) => Ok(obj),
			None => Err(Error::from("function '{func_name}' doesn't return a value")),
		}
	}

	fn expression(&mut self, local_scope: &mut Option<&mut VariableScope>, e: Expr) -> Result<Object, Error> {
		match e {
			Expr::Binary(left, op, right) => {
				let x = self.expression(local_scope, *left)?;
				let y = self.expression(local_scope, *right)?;

				match (x, op, y) {
					(Object::Number(x), Token::Plus, Object::Number(y)) => Ok(Object::Number(x + y)),
					(Object::Number(x), Token::Minus, Object::Number(y)) => Ok(Object::Number(x - y)),
					(Object::Number(x), Token::Star, Object::Number(y)) => Ok(Object::Number(x * y)),
					(Object::Number(x), Token::Slash, Object::Number(y)) => {
						if y == 0.0 {
							Err(Error::from("division by zero"))
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

					_ => Err(Error::from("invalid binary operands")),
				}
			}

			Expr::Unary(op, right) => {
				let x = self.expression(local_scope, *right)?;

				match (op, x) {
					(Token::Minus, Object::Number(x)) => Ok(Object::Number(-x)),
					(Token::Plus, Object::Number(x)) => Ok(Object::Number(x)),

					_ => Err(Error::from("invalid unary operands")),
				}
			}

			Expr::Group(expr) => self.expression(local_scope, *expr),

			Expr::Call(func_name, args) => self.call(local_scope, func_name, args),

			Expr::Number(x) => Ok(Object::Number(x)),

			Expr::Bool(b) => Ok(Object::Bool(b)),

			Expr::Var(var_name) => {
				match local_scope {
					Some(local_scope) => match local_scope.get(&var_name) {
						Some(obj) => Ok(obj.clone()),
						None => Err(Error::new(format!("variable '{var_name}' not found"))),
					}

					None => match self.global_scope.get(&var_name) {
						Some(obj) => Ok(obj.clone()),
						None => Err(Error::new(format!("variable '{var_name}' not found"))),
					}
				}
			}
		}
	}

	fn body(&mut self, local_scope: &mut Option<&mut VariableScope>, body: Vec<Stmt>) -> Result<Option<Object>, Error> {
		for stmt in body {
			match stmt {
				Stmt::If(cond, body, else_body) => {
					let cond = self.expression(local_scope, cond)?;
					let ret = match (cond, else_body) {
						(Object::Bool(true), _) => self.body(local_scope, body),

						(Object::Bool(false), Some(else_body)) => self.body(local_scope, else_body),

						(Object::Bool(false), None) => Ok(None),

						_ => { return Err(Error::from("invalid expression for if statement")) }
					};

					if let Ok(Some(_)) = ret {
						return ret;
					}
				}

				Stmt::Func(func_name, args, body) => {
					match local_scope {
						Some(_) => { return Err(Error::from("function definitions musn't be nested")); }

						None => {
							let func = Func{args: args, body: body};
							self.func_table.insert(func_name, func);
						}
					}
				}

				Stmt::Assign(var_name, expr) => {
					let val = self.expression(local_scope, expr)?;

					match local_scope {
						Some(local_scope) => { local_scope.insert(var_name, val); }
						None => { self.global_scope.insert(var_name, val); } 
					}
				}

				Stmt::Print(expr) => {
					let obj = self.expression(local_scope, expr)?; 
					println!("{obj}");
				}

				Stmt::Return(expr) => {
					match local_scope {
						Some(_) => {
							let obj = self.expression(local_scope, expr)?; 
							return Ok(Some(obj));
						}

						None => { return Err(Error::from("can't return from global scope")); }
					}
				}
			}
		}

		Ok(None)
	}

	pub fn eval(&mut self, stmts: Vec<Stmt>) -> Result<(), Error> {
		self.body(&mut None, stmts)?;

		Ok(())
	}
}

