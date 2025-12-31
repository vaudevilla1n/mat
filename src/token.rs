/*
   Tokens contain their line and line column
 */

pub enum Token {
	ILLEGAL(String), EOF,

		LParen, RParen,
		LBracket, RBracket,

		Plus, Minus, Slash, Star, Comma, Semicolon,

		Equal, EqualEqual,
		Greater, GreaterEqual,
		Less, LessEqual,

		If, Else, Return, Print,

		True, False,

		Identifier(String), Number(f64),
}
