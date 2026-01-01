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
	eof: bool
}

#[allow(dead_code)]
impl<'s> Lexer<'s> {
	pub fn new(src: &'s str) -> Lexer<'s> {
		Lexer{
			src: src,
			chars: src.char_indices().peekable(),
			col: 1,
			line: 1,
			eof: false,
		}
	}

	fn skip_whitespace(&mut self) {
		while let Some((_, c)) = self.chars.next_if(|&(_, c)| c.is_whitespace()) {
			if c == '\n' {
				self.line += 1;
				self.col = 1;
			} else {
				self.col += 1;
			}
		}
	}

	fn slice(&mut self, start: usize) -> &'s str {
		match self.chars.peek() {
			Some(&(i, _)) => &self.src[start..i],
			None => &self.src[start..],
		}
	}

	fn number(&mut self, start: usize) -> Token {
		while self.chars.next_if(|&(_, c)| c.is_numeric()) != None { }
		if self.chars.next_if(|&(_, c)| c == '.') != None {
			while self.chars.next_if(|&(_, c)| c.is_numeric()) != None { }
		}

		let s = self.slice(start);
		let x: f64 = s.parse().unwrap();

		Token::Number(x)
	}

	fn identifier(&mut self, start: usize) -> Token {
		while self.chars.next_if(|&(_, c)| c.is_alphanumeric() || c == '_') != None { }

		let s = self.slice(start);

		match s {
		"if" => Token::If,
		"else" => Token::Else,
		"return" => Token::Return,
		"print" => Token::Print,
		"true" => Token::True,
		"false" => Token::False,

		_ => Token::Identifier(String::from(s)),
		}
	}
}

impl<'s> Iterator for Lexer<'s> {
	type Item = (Token, &'s str, usize, usize);

	#[allow(unused_variables)]
	fn next(&mut self) -> Option<Self::Item> {
		if self.eof {
			return None;
		}

		self.skip_whitespace();
		
		let (line, col) = (self.line, self.col);

		self.col += 1;
		let (start, tok) = if let Some((i, c)) = self.chars.next() {
			let t = match c {
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

				'=' => match self.chars.next_if(|&(_, c)| c == '=') {
					None => Token::Assign,
					Some(_) => Token::Equal,
				}

				'>' => match self.chars.next_if(|&(_, c)| c == '=') {
					None => Token::Greater,
					Some(_) => Token::GreaterEqual,
				}

				'<' => match self.chars.next_if(|&(_, c)| c == '=') {
					None => Token::Less,
					Some(_) => Token::LessEqual,
				}

				('0'..='9') => self.number(i),
				
				('a'..='z') | ('A'..='Z') | '_' => self.identifier(i),

				_ => Token::Illegal(String::from("unknown token")),
			};

			(i, t)
		} else {
			self.eof = true;
			return Some((Token::EOF, "EOF", line, col));
		};

		let text = self.slice(start);

		Some((tok, text, line, col))
	}
}
