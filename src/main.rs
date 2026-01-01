use std::{
	env, process, fs, path::Path
};

pub mod lexer;

fn run(src: &str) {
	let l = lexer::Lexer::new(src);

	for (t, line, col) in l {
		match t {
		lexer::Token::Illegal(err) => {
			eprintln!("{err}");
			return;
		}

		lexer::Token::EOF => { return; }

		_ => println!("{t:?}: line {line}: col {col}"),
		}
	}
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
