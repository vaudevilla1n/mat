use crate::token::Token;

pub struct Lexer {
	src: String,
	pos: usize,
	line: usize,
	linecol: usize,
}

impl Lexer {
	pub fn new(src: String) -> Lexer {
		Lexer { src: src, pos: 0, line: 1, linecol: 1 }
	}

	fn peek(&self) -> Option<char> {
		self.src.chars().nth(self.pos)
	}

	fn advance(&mut self) -> Option<char> {
		let c = self.src.chars().nth(self.pos);

		if c == None {
			return None
		}

		let c = c.unwrap();

		if c == '\n' {
			self.line += 1;
			self.linecol = 1;
		} else {
			self.linecol += 1;
		}

		Some(c)
	}

	pub fn next(&mut self) -> (Token, usize, usize) {
		let line = self.line;
		let linecol = self.linecol;
		let token = match self.src.as_bytes() {
			[b'(', rest @ ..] => { self.src = rest; Token::LPAREN },
				/*
				   ')' => token::RPAREN,
				   '{' => token::LBRACKET,
				   '}' => token::RBRACKET,
				   '+' => token::PLUS,
				   '-' => token::MINUS,
				   '*' => token::STAR,
				   '/' => token::SLASH,
				   ',' => token::COMMA,
				   ';' => token::SEMICOLON,
				 */
				[b'>', b'=', rest @ ..] => { self.src = rest; Token::LPAREN },
				_ => panic!("bad")
		};

		(token, line, linecol)
	}
}
