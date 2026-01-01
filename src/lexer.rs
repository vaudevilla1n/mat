use std::{
	str::CharIndices,
	iter::Peekable,
};

#[derive(Debug)]
pub enum Token{
	Illegal(String),
	EOF,

	LParen,
	RParen,
	LBracket,
	RBracket,

	Plus,
	Minus,
	Slash,
	Star,
	Comma,	
	Semicolon,

	Assign,
	Equal,
	Greater,
	GreaterEqual,
	Less,
	LessEqual,

	If,
	Else,
	Return,
	Print,

	True,
	False,

	Identifier(String),
	Number(f64),
}

#[allow(dead_code)]
pub struct Lexer<'s> {
	src: &'s str,
	chars: Peekable<CharIndices<'s>>,
	col: usize,
	line: usize,
}

#[allow(dead_code)]
impl<'s> Lexer<'s> {
	pub fn new(src: &'s str) -> Lexer<'s> {
		Lexer{
			src: src,
			chars: src.char_indices().peekable(),
			col: 1,
			line: 1,
		}
	}

	fn skip_whitespace(&mut self) {
		while let Some(&(_, c)) = self.chars.peek() {
			if !c.is_whitespace() {
				break;
			}
	
			if c == '\n' {
				self.line += 1;
				self.col = 1;
			} else {
				self.col += 1;
			}

			self.chars.next();
		}
	}
}

impl<'s> Iterator for Lexer<'s> {
	type Item = (Token, usize, usize);

	#[allow(unused_variables)]
	fn next(&mut self) -> Option<Self::Item> {
		self.skip_whitespace();
		
		let (line, col) = (self.line, self.col);
		let tok = if let Some(&(i, c)) = self.chars.peek() {
			match c {
				'(' => Token::LParen,
				')' => Token::RParen,
				'{' => Token::LBracket,
				'}' => Token::RBracket,

				'+' => Token::Plus,
				'-' => Token::Minus,
				'/' => Token::Slash,
				'*' => Token::Star,
				',' => Token::Comma,
				';' => Token::Semicolon,

				'=' => Token::Assign,
				'>' => Token::Greater,
				'<' => Token::Less,

				_ => Token::Illegal(String::from("unknown token")),
			}
		} else {
			Token::EOF
		};
		self.col += 1;
		self.chars.next();

		Some((tok, line, col))
	}
}
