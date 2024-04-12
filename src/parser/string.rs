use std::fmt::{Debug, Display, Formatter};

use logos::{Lexer, Logos};

use crate::parser::{ast::ShortEscapeType, token::LexerError};

pub(crate) fn parse_unicode_escape(source: &str) -> Result<char, LexerError> {
	let char_code: &str = source.get(2..).unwrap();
	char::try_from(u32::from_str_radix(char_code, 16)?).map_err(LexerError::CharLiteralOverflow)
}

fn parse_short_escape(lex: &mut Lexer<StringToken>) -> Result<ShortEscapeType, LexerError> {
	Ok(match lex.slice().trim_matches('\\') {
		"n" => ShortEscapeType::Newline,
		"r" => ShortEscapeType::CarriageReturn,
		"t" => ShortEscapeType::Tab,
		"\\" => ShortEscapeType::Backslash,
		"\"" => ShortEscapeType::Quote,
		"$" => ShortEscapeType::Dollar,
		_ => unreachable!(),
	})
}

#[derive(Logos, Clone, Debug, PartialEq)]
#[logos(error = LexerError)]
pub(crate) enum StringToken {
	#[regex(r"\\u[\da-fA-F]{1,4}", |lex| parse_unicode_escape(lex.slice()))]
	UnicodeEscape(char),

	#[regex(r#"\\[nrt"\\]"#, parse_short_escape)]
	ShortEscape(ShortEscapeType),

	#[regex(r#"[^"\\]"#, |lex| lex.slice().parse())]
	Grapheme(char),

	// #[regex(r#"$\{.+\}"#, parse_expr)]
	// InterpolatedExpr(),
	#[token("\"")]
	Quote,
}

impl Display for StringToken {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}",
			match self {
				StringToken::UnicodeEscape(val) | StringToken::Grapheme(val) => val.to_string(),
				StringToken::ShortEscape(val) => val.character().to_string(),
				StringToken::Quote => String::from("<string boundary>"),
			}
		)
	}
}
