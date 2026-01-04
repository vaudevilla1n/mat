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
	local_scope: Vec<VariableScope>,
}

impl Runtime {
	pub fn new() -> Runtime {
		Runtime{
			func_table: HashMap::new(),
			global_scope: HashMap::new(),
			local_scope: Vec::new(),
		}
	}

	fn call(&mut self, func_name: String, input: Vec<Expr>) -> Result<Object, Error> {
		let func = match self.func_table.get(&func_name) {
			Some(func) => func.clone(),
			None => { return Err(Error::new(format!("function '{func_name}' doesn't exist"))); }
		};

		if input.len() != func.args.len() {
			return Err(Error::new(format!("function '{func_name}' expects {} arg(s) but got {} arg(s)", func.args.len(), input.len()))); 
		}

		let mut func_scope: VariableScope = HashMap::new();
		for (arg, input) in iter::zip(func.args, input) {
			let val = self.expression(input)?;
			func_scope.insert(arg, val);
		}

		self.local_scope.push(func_scope);
		let ret = self.body(func.body)?;
		self.local_scope.pop();

		match ret {
			Some(obj) => Ok(obj),
			None => Err(Error::new(format!("function '{func_name}' doesn't return a value"))),
		}
	}

	fn expression(&mut self, e: Expr) -> Result<Object, Error> {
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
				let x = self.expression(*right)?;

				match (op, x) {
					(Token::Minus, Object::Number(x)) => Ok(Object::Number(-x)),
					(Token::Plus, Object::Number(x)) => Ok(Object::Number(x)),

					_ => Err(Error::from("invalid unary operands")),
				}
			}

			Expr::Group(expr) => self.expression(*expr),

			Expr::Call(func_name, args) => self.call(func_name, args),

			Expr::Number(x) => Ok(Object::Number(x)),

			Expr::Bool(b) => Ok(Object::Bool(b)),

			Expr::Var(var_name) => {
				if let Some(local_scope) = self.local_scope.last() {
					if let Some(obj) = local_scope.get(&var_name) {
						return Ok(obj.clone());
					}
				}

				match self.global_scope.get(&var_name) {
					Some(obj) => Ok(obj.clone()),
					None => Err(Error::new(format!("variable '{var_name}' not found"))),
				}
			}
		}
	}

	fn body(&mut self, body: Vec<Stmt>) -> Result<Option<Object>, Error> {
		for stmt in body {
			match stmt {
				Stmt::If(cond, body, else_body) => {
					let cond = self.expression(cond)?;
					let ret = match (cond, else_body) {
						(Object::Bool(true), _) => self.body(body),

						(Object::Bool(false), Some(else_body)) => self.body(else_body),

						(Object::Bool(false), None) => Ok(None),

						_ => { return Err(Error::from("invalid expression for if statement")) }
					};

					if let Ok(Some(_)) = ret {
						return ret;
					}
				}

				Stmt::Func(func_name, args, body) => {
					match self.local_scope.last() {
						Some(_) => { return Err(Error::from("function definitions musn't be nested")); }

						None => {
							let func = Func{args: args, body: body};
							match self.func_table.get(&func_name) {
								Some(_) => { return Err(Error::new(format!("redefinition of function '{func_name}'"))); } 
								None => { self.func_table.insert(func_name, func); }
							}
						}
					}
				}

				Stmt::Assign(var_name, expr) => {
					let val = self.expression(expr)?;

					match self.local_scope.last_mut() {
						Some(local_scope) => { local_scope.insert(var_name, val); }
						None => { self.global_scope.insert(var_name, val); } 
					}
				}

				Stmt::Print(expr) => {
					let obj = self.expression(expr)?; 
					println!("{obj}");
				}

				Stmt::Return(expr) => {
					match self.local_scope.last() {
						Some(_) => {
							let obj = self.expression(expr)?; 
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
		self.body(stmts)?;

		Ok(())
	}
}

