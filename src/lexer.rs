use std::{
	str::CharIndices,
	iter::Peekable,
};

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub enum Token {
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
pub struct Lexer<'s> {
	src: &'s str,
	chars: Peekable<CharIndices<'s>>,
	col: usize,
	line: usize,
	curr: Token
}

impl<'s> Lexer<'s> {
	pub fn new(src: &'s str) -> Lexer<'s> {
		let mut lexer = Lexer{
			src: src,
			chars: src.char_indices().peekable(),
			col: 1,
			line: 1,
			curr: Token::EOF,
		};
		lexer.next();

		lexer
	}

	pub fn pos(&self) -> (usize, usize) {
		(self.line, self.col)
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

	fn advance_if(&mut self, c: char) -> bool {
		match self.chars.next_if(|&(_, curr)| curr == c) {
			Some(_) => {
				self.col += 1;

				true
			},

			None => false,
		}
	}

	fn text_from(&mut self, start: usize) -> &'s str {
		match self.chars.peek() {
			Some(&(i, _)) => &self.src[start..i],
			None => &self.src[start..],
		}
	}

	fn number(&mut self, start: usize) -> Token {
		let is_num = |(_, c): &(usize, char)| c.is_numeric();

		while self.chars.next_if(is_num) != None {
			self.col += 1;
		}

		if self.advance_if('.') {
			while self.chars.next_if(is_num) != None {
				self.col += 1;
			}
		}

		let x: f64 = self.text_from(start).parse().unwrap();

		Token::Number(x)
	}

	fn identifier(&mut self, start: usize) -> Token {
		let is_alnum = |(_i, c): &(usize, char)| c.is_alphanumeric();

		while self.chars.next_if(is_alnum) != None {
			self.col += 1;
		}

		let s = self.text_from(start);

		match s {
		"fn" => Token::Func,
		"if" => Token::If,
		"else" => Token::Else,
		"return" => Token::Return,
		"print" => Token::Print,
		"true" => Token::True,
		"false" => Token::False,

		_ => Token::Identifier(String::from(s)),
		}
	}

	pub fn next(&mut self) -> Token {
		self.skip_whitespace();

		let curr = self.curr.clone();
		self.curr = if let Some((i, c)) = self.chars.next() {
			let is_num = |(_, c): &(usize, char)| c.is_numeric();
			self.col += 1;

			match c {
				'(' => Token::LParen,
				')' => Token::RParen,
				'{' => Token::LBracket,
				'}' => Token::RBracket,

				'+' => if self.chars.next_if(is_num) == None {
					Token::Plus
				} else {
					self.number(i)
				}

				'-' => if self.chars.next_if(is_num) == None {
					Token::Minus
				} else {
					self.number(i)
				}

				'/' => Token::Slash,
				'*' => Token::Star,
				'^' => Token::Caret,
				',' => Token::Comma,
				';' => Token::Semicolon,

				'=' => if !self.advance_if('=') {
					Token::Assign
				} else {
					Token::Equal
				}

				'>' => if !self.advance_if('=') {
					Token::Greater
				} else {
					Token::GreaterEqual
				}

				'<' => if !self.advance_if('=') {
					Token::Less
				} else {
					Token::LessEqual
				}

				('0'..='9') => self.number(i),
				
				('a'..='z') | ('A'..='Z') | '_' => self.identifier(i),

				_ => Token::Illegal(String::from("erroneous character")),
			}
		} else {
			Token::EOF
		};

		curr
	}

	pub fn peek(&self) -> Token {
		self.curr.clone()
	}
}
