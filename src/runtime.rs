use std::{ fmt, error, collections::HashMap };
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

struct Func {
	args: Vec<String>,
	body: Stmt,
	scope: VariableScope,
}

pub struct Runtime<'l> {
	func_table: FuncTable,
	global_scope: VariableScope,
	local_scope: Option<&'l VariableScope>,
}

impl<'l> Runtime<'l> {
	pub fn new() -> Runtime<'l> {
		Runtime{
			func_table: HashMap::new(),
			global_scope: HashMap::new(),
			local_scope: None,
		}
	}

	fn get_variable(&self, var_name: &String) -> Option<Object> {
		if let Some(local_scope) = self.local_scope && let Some(obj) = local_scope.get(var_name) {
			Some(obj.clone())
		} else if let Some(obj) = self.global_scope.get(var_name) {
			Some(obj.clone())
		} else {
			None
		}
	}

	fn insert_variable(&mut self, var_name: String, val: Object) {
		if let Some(mut local_scope) = self.local_scope {
			local_scope.insert(var_name, val);
		} else {
			self.global_scope.insert(var_name, val);
		}
	}

	fn call(&self, func_name: String, args: Vec<Expr>) -> Result<Object, Error> {
		let func = match self.func_table.get(&func_name) {
			Some(func) => &mut func,
			None => { return Err(Error::new(format!("function '{}' not found", func_name))); }
		};

		let expected_argc = func.args.len();
		let argc = args.len();

		if argc != expected_argc {
			let err = format!("function '{}': expected {} arg(s), got '{}' arg(s)", func_name, expected_argc, argc);
			return Err(Error::new(err));
		}

		for (i, arg) in args.into_iter().enumerate() {
			let val = self.expression(arg)?;
			func.scope.insert(func.args[i], val);
		}

		let prev_scope = self.local_scope;
		self.local_scope = Some(&func.scope);

		let ret = self.body(func.body)?;
		self.local_scope = prev_scope;

		match ret {
			Some(obj) => Ok(obj),
			None => Err(Error::new(format!("function '{func_name}' does not return a value"))),
		}
	}

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
				match self.get_variable(&var_name) {
					Some(obj) => Ok(obj),
					None => Err(Error::new(format!("variable '{var_name}' not found"))),
				}
			}
		}
	}

	fn body(&mut self, body: Stmt) -> Result<Option<Object>, Error> {
		let body = match body {
			Stmt::Body(body) => body,
			_ => unreachable!(),
		};

		for stmt in body {
			let ret = match stmt {
				Stmt::Body(_) => self.body(stmt)?,

				Stmt::If(cond, body, else_body) => self.if_stmt(cond, body, else_body)?,

				Stmt::Func(_, _, _) => { return Err(Error::from("function definitions musn't be nested")); }

				Stmt::Assign(var_name, expr) => {
					let val = self.expression(expr)?;
					self.insert_variable(var_name, val); 

					None
				}

				Stmt::Print(expr) => {
					let obj = self.expression(expr)?; 
					println!("{obj}");

					None
				}

				Stmt::Return(expr) => {
					let obj = self.expression(expr)?; 

					Some(obj)
				}
			};

			if let Some(ret) = ret {
				return Ok(Some(ret));
			}
		}

		Ok(None)
	}

	fn if_stmt(&mut self, cond: Expr, body: Box<Stmt>, else_body: Option<Box<Stmt>>) -> Result<Option<Object>, Error> {
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
					Ok(None)
				}
			}

			_ => Err(Error::from("invalid expression for if statement")),
		}
	}

	fn func_stmt(&mut self, func_name: String, args: Vec<String>, body: Box<Stmt>) -> Result<(), Error> {
		let func = Func{args: args, body: *body, scope: HashMap::new()};
		self.func_table.insert(func_name, func);

		Ok(())
	}

	#[allow(unused_variables)]
	pub fn eval(&mut self, stmts: Vec<Stmt>) -> Result<(), Error> {
		for stmt in stmts {
			println!("{stmt:?}");

			match stmt {
				Stmt::If(cond, body, else_body) => { self.if_stmt(cond, body, else_body)?; }

				Stmt::Func(func_name, args, body) => { self.func_stmt(func_name, args, body)?; }

				Stmt::Assign(var_name, expr) => {
					let val = self.expression(expr)?;
					self.insert_variable(var_name, val);
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

