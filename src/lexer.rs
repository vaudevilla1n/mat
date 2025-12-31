use crate::token;

pub mod lexer;

struct Lexer {
	src: String
	pos: usize
	line: usize:
	linecol: usize
}
