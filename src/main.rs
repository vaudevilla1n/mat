use std::{
	env, process, fs, path::Path
};

pub mod lexer;
pub mod parser;

fn run(src: &str) {
	let lexer = lexer::Lexer::new(src);
	let mut parser = parser::Parser::new(lexer); 

	parser.parse();
}

fn main() {
	let mut args = env::args();
	if args.len() != 2 {
			eprintln!("usage: ./mat FILE");
			process::exit(1);
	}

	let file = args.nth(1).unwrap();
	match fs::read_to_string(Path::new(&file)) {
		Ok(src) => {
			run(&src)
		}

		Err(err) => {
			eprintln!("error: \"{file}\": {err}");
			process::exit(1);
		}
	}
}
