use std::{
	str::CharIndices,
	iter::Peekable,
};

#[derive(Clone)]
#[derive(Debug)]
pub enum TokenKind {
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
	Caret,
	Comma,	
	Semicolon,

	Assign,
	Equal,
	Greater,
	GreaterEqual,
	Less,
	LessEqual,

	Func,
	If,
	Else,
	Return,
	Print,

	True,
	False,

	Identifier(String),
	Number(f64),
}

#[derive(Clone)]
#[derive(Debug)]
pub struct Token<'source> {
	pub kind: TokenKind,
	pub text: &'source str,
	pub col: usize,
	pub line: usize,
}

#[derive(Clone)]
#[derive(Debug)]
pub struct Lexer<'s> {
	src: &'s str,
	chars: Peekable<CharIndices<'s>>,
	col: usize,
	line: usize,
	eof: bool
}

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

	fn number(&mut self, start: usize) -> TokenKind {
		while self.chars.next_if(|&(_, c)| c.is_numeric()) != None { }
		if self.chars.next_if(|&(_, c)| c == '.') != None {
			while self.chars.next_if(|&(_, c)| c.is_numeric()) != None { }
		}

		let s = self.slice(start);
		let x: f64 = s.parse().unwrap();

		TokenKind::Number(x)
	}

	fn identifier(&mut self, start: usize) -> TokenKind {
		while self.chars.next_if(|&(_, c)| c.is_alphanumeric() || c == '_') != None { }

		let s = self.slice(start);

		match s {
		"fn" => TokenKind::Func,
		"if" => TokenKind::If,
		"else" => TokenKind::Else,
		"return" => TokenKind::Return,
		"print" => TokenKind::Print,
		"true" => TokenKind::True,
		"false" => TokenKind::False,

		_ => TokenKind::Identifier(String::from(s)),
		}
	}
}

impl<'s> Iterator for Lexer<'s> {
	type Item = Token<'s>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.eof {
			return None;
		}

		self.skip_whitespace();
		
		let (line, col) = (self.line, self.col);

		self.col += 1;
		let (start, kind) = if let Some((i, c)) = self.chars.next() {
			let k = match c {
				'(' => TokenKind::LParen,
				')' => TokenKind::RParen,
				'{' => TokenKind::LBracket,
				'}' => TokenKind::RBracket,

				'+' => match self.chars.next_if(|&(_, c)| c.is_numeric()) {
					None => TokenKind::Plus,
					Some(_) => self.number(i),
				}

				'-' => match self.chars.next_if(|&(_, c)| c.is_numeric()) {
					None => TokenKind::Minus,
					Some(_) => self.number(i),
				}

				'/' => TokenKind::Slash,
				'*' => TokenKind::Star,
				'^' => TokenKind::Caret,
				',' => TokenKind::Comma,
				';' => TokenKind::Semicolon,

				'=' => match self.chars.next_if(|&(_, c)| c == '=') {
					None => TokenKind::Assign,
					Some(_) => TokenKind::Equal,
				}

				'>' => match self.chars.next_if(|&(_, c)| c == '=') {
					None => TokenKind::Greater,
					Some(_) => TokenKind::GreaterEqual,
				}

				'<' => match self.chars.next_if(|&(_, c)| c == '=') {
					None => TokenKind::Less,
					Some(_) => TokenKind::LessEqual,
				}

				('0'..='9') => self.number(i),
				
				('a'..='z') | ('A'..='Z') | '_' => self.identifier(i),

				_ => TokenKind::Illegal(String::from("erroneous character")),
			};

			(i, k)
		} else {
			self.eof = true;
			return Some(Token{kind: TokenKind::EOF, text: "EOF", col: col, line: line});
		};

		let text = self.slice(start);

		Some(Token{kind: kind, text: text, col: col, line: line})
	}
}
