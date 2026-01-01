use std::{ iter::Peekable };
use crate::lexer;

#[derive(Debug)]
pub struct Parser<'s> {
	tokens: Peekable<lexer::Lexer<'s>>
}

impl<'s> Parser<'s> {
	pub fn new(l: lexer::Lexer<'s>) -> Parser<'s> {
		Parser{tokens: l.peekable()}
	}

	pub fn parse(&mut self) {
		loop {
			match self.tokens.next() {
			Some(tok) => {
				match tok.kind {
					lexer::TokenKind::Illegal(err) => {
						eprintln!("{err}");
						break;
					}

					lexer::TokenKind::EOF => break,

					_ => {
						println!("{:?} \'{}\' (line {}, col {})", tok.kind, tok.text, tok.line, tok.col);
					}
				}
			}

			None => break,
			}
		}
	}
}
