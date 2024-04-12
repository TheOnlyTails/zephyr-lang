use itertools::Itertools;
use std::{
	char::{CharTryFromError, ParseCharError},
	fmt::{Display, Formatter},
	num::{ParseFloatError, ParseIntError},
};

use logos::{Lexer, Logos};
use ordered_float::OrderedFloat;
use regex::Regex;
use thiserror::Error;

use crate::{
	parser::{ast::StringComponent, lexer::Lexer as SpannedLexer, string::StringToken},
	utils::IntoOk,
};

pub(crate) fn parse_char(input: &str) -> Result<char, LexerError> {
	let content = &input[1..input.len() - 1];

	if content.starts_with("\\u")
		&& let Some(caps) = Regex::new(r"\\u([\da-fA-F]{1,4})")
			.unwrap()
			.captures(content)
	{
		let char_code: &str = caps.get(1).unwrap().into();
		return char::try_from(u32::from_str_radix(char_code, 16)?)
			.map_err(LexerError::CharLiteralOverflow);
	}

	match content {
		"\\n" => '\n'.ok(),
		"\\r" => '\r'.ok(),
		"\\t" => '\t'.ok(),
		"\\\\" => '\\'.ok(),
		"\\'" => '\''.ok(),
		_ => content.parse().map_err(LexerError::InvalidCharLiteral),
	}
}

pub(crate) fn parse_string_literal<'input>(
	source: Lexer<'input, Token<'input>>,
) -> Result<Vec<StringComponent<'input>>, LexerError> {
	let mut result = vec![];

	for token in SpannedLexer::<StringToken>::new(source.source()) {
		match token?.1 {
			StringToken::UnicodeEscape(val) => result.push(StringComponent::UnicodeEscape(val)),
			StringToken::ShortEscape(val) => result.push(StringComponent::ShortEscape(val)),
			StringToken::Grapheme(val) => result.push(StringComponent::Grapheme(val)),
			// StringToken::InterpolatedExpr() => {}
			StringToken::Quote => {}
		}
	}

	Ok(result)
}

#[derive(Logos, Clone, Debug, PartialEq)]
#[logos(skip r"\s+", skip r"//.*\n?", error = LexerError)]
pub(crate) enum Token<'input> {
	// Literals
	#[regex(r"\d[\d_]*", |lex| lex.slice().replace('_', "").parse(), priority = 11)]
	#[regex(r"0b[01][01_]*", |lex| u64::from_str_radix(&lex.slice().trim_start_matches("0b").replace('_', ""), 2))]
	#[regex(r"0o[0-7][0-7_]*", |lex| u64::from_str_radix(&lex.slice().trim_start_matches("0o").replace('_', ""), 8))]
	#[regex(r"0x[\da-fA-F][\da-fA-F_]*", |lex| u64::from_str_radix(&lex.slice().trim_start_matches("0x").replace('_', ""), 16))]
	Int(u64),

	#[regex(r"\d*\.\d+(?:[eE][+-]?\d+)?", |lex| lex.slice().parse())]
	Float(OrderedFloat<f64>),

	#[regex(r"'\\u[\da-fA-F]{1,4}'", |lex| parse_char(lex.slice()))]
	#[regex(r"'\\[nrt'\\]'", |lex| parse_char(lex.slice()))]
	#[regex(r"'[^'\\]'", |lex| parse_char(lex.slice()))]
	Char(char),

	#[regex(r#""(\\u[\da-fA-F]{1,4}|\\[nrt"\\]|[^"\\])*""#, |lex| parse_string_literal(lex.to_owned()))]
	String(Vec<StringComponent<'input>>),

	#[token("false", |_| false)]
	#[token("true", |_| true)]
	Boolean(bool),

	#[regex(r"\p{XID_Start}\p{XID_Continue}*", Lexer::slice)]
	Identifier(&'input str),

	#[token("#[")]
	ListStart,
	#[token("#{")]
	MapStart,
	#[token("#(")]
	TupleStart,
	#[token("{|")]
	ObjectStart,
	#[token("|}")]
	ObjectEnd,

	// Keywords
	#[token("let")]
	Let,
	#[token("func")]
	Func,
	#[token("struct")]
	Struct,
	#[token("mut")]
	Mut,
	#[token("if")]
	If,
	#[token("else")]
	Else,
	#[token("while")]
	While,
	#[token("for")]
	For,
	#[token("import")]
	Import,
	#[token("with")]
	With,
	#[token("as")]
	As,
	#[token("return")]
	Return,

	// Types
	#[token("int")]
	IntType,
	#[token("float")]
	FloatType,
	#[token("string")]
	StringType,
	#[token("char")]
	CharType,
	#[token("boolean")]
	BooleanType,
	#[token("void")]
	VoidType,

	// Operators
	#[token("+")]
	Plus,
	#[token("-")]
	Minus,
	#[token("*")]
	Times,
	#[token("/")]
	Divide,
	#[token("%")]
	Modulus,
	#[token("**")]
	Power,
	#[token("&&")]
	And,
	#[token("||")]
	Or,
	#[token("&")]
	BitAnd,
	#[token("|")]
	BitOr,
	#[token("^")]
	BitXor,
	#[token("<<")]
	LeftShift,
	#[token(">>")]
	RightShift,
	#[token("..")]
	RangeTo,
	#[token("..=")]
	RangeToEquals,
	#[token("is")]
	Is,
	#[token("!is")]
	NotIs,

	#[token(">")]
	GreaterThan,
	#[token(">=")]
	GreaterEquals,
	#[token("<")]
	LessThan,
	#[token("<=")]
	LessEquals,
	#[token("==")]
	Equals,
	#[token("!=")]
	NotEquals,

	#[token("+=")]
	PlusAssign,
	#[token("-=")]
	MinusAssign,
	#[token("*=")]
	TimesAssign,
	#[token("/=")]
	DivideAssign,
	#[token("%=")]
	ModulusAssign,
	#[token("**=")]
	PowerAssign,
	#[token("&&=")]
	AndAssign,
	#[token("||=")]
	OrAssign,
	#[token("&=")]
	BitAndAssign,
	#[token("|=")]
	BitOrAssign,
	#[token("^=")]
	BitXorAssign,
	#[token("<<=")]
	LeftShiftAssign,
	#[token(">>=")]
	RightShiftAssign,

	#[token("->")]
	Arrow,
	#[token("in")]
	In,
	#[token("!in")]
	NotIn,
	#[token("++")]
	Increment,
	#[token("--")]
	Decrement,

	// Punctuation
	#[token("(")]
	LeftParen,
	#[token(")")]
	RightParen,
	#[token("[")]
	LeftBracket,
	#[token("]")]
	RightBracket,
	#[token("{")]
	LeftBrace,
	#[token("}")]
	RightBrace,
	#[token("!")]
	Exclamation,
	#[token("?")]
	QuestionMark,
	#[token(".")]
	Period,
	#[token(",")]
	Comma,
	#[token("\\")]
	Backslash,
	#[token("@")]
	AtSign,
	#[token("#")]
	Hash,
	#[token("$")]
	DollarSign,
	#[token("_")]
	Underscore,
	#[token("=")]
	EqualSign,
	#[token("'")]
	Apostrophe,
	#[token("\"")]
	Quote,
	#[token("~")]
	Tilde,
	#[token("`")]
	Backtick,
	#[token(";")]
	Semicolon,
	#[token(":")]
	Colon,
}

impl Display for Token<'_> {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}",
			match self {
				Token::Int(val) => val.to_string(),
				Token::Float(val) => val.to_string(),
				Token::Char(val) => val.to_string(),
				Token::String(parts) => format!(
					r#""{}""#,
					parts
						.iter()
						.map(|part| match part {
							StringComponent::Grapheme(val) => val.to_string(),
							StringComponent::ShortEscape(val) => val.character().to_string(),
							StringComponent::UnicodeEscape(val) => val.to_string(),
							StringComponent::InterpolatedExpr(expr) => format!("${{{:?}}}", expr),
						})
						.join("")
				),
				Token::Boolean(val) => val.to_string(),
				Token::Identifier(val) => val.to_string(),
				Token::ListStart => "#[".to_string(),
				Token::MapStart => "#{".to_string(),
				Token::TupleStart => "#(".to_string(),
				Token::ObjectStart => "{|".to_string(),
				Token::ObjectEnd => "|}".to_string(),
				Token::Let => "let".to_string(),
				Token::Func => "func".to_string(),
				Token::Struct => "struct".to_string(),
				Token::Mut => "mut".to_string(),
				Token::If => "if".to_string(),
				Token::Else => "else".to_string(),
				Token::While => "while".to_string(),
				Token::For => "for".to_string(),
				Token::Import => "import".to_string(),
				Token::With => "with".to_string(),
				Token::As => "as".to_string(),
				Token::Return => "return".to_string(),
				Token::IntType => "int".to_string(),
				Token::FloatType => "float".to_string(),
				Token::StringType => "string".to_string(),
				Token::CharType => "char".to_string(),
				Token::BooleanType => "boolean".to_string(),
				Token::VoidType => "void".to_string(),
				Token::Plus => "+".to_string(),
				Token::Minus => "-".to_string(),
				Token::Times => "*".to_string(),
				Token::Divide => "/".to_string(),
				Token::Modulus => "%".to_string(),
				Token::Power => "**".to_string(),
				Token::And => "&&".to_string(),
				Token::Or => "||".to_string(),
				Token::BitAnd => "&".to_string(),
				Token::BitOr => "|".to_string(),
				Token::BitXor => "^".to_string(),
				Token::LeftShift => "<<".to_string(),
				Token::RightShift => ">>".to_string(),
				Token::RangeTo => "..".to_string(),
				Token::RangeToEquals => "..=".to_string(),
				Token::In => "in".to_string(),
				Token::NotIn => "!in".to_string(),
				Token::Is => "is".to_string(),
				Token::NotIs => "!is".to_string(),
				Token::GreaterThan => ">".to_string(),
				Token::GreaterEquals => ">=".to_string(),
				Token::LessThan => "<".to_string(),
				Token::LessEquals => "<=".to_string(),
				Token::Equals => "==".to_string(),
				Token::NotEquals => "!=".to_string(),
				Token::PlusAssign => "+=".to_string(),
				Token::MinusAssign => "-=".to_string(),
				Token::TimesAssign => "*=".to_string(),
				Token::DivideAssign => "/=".to_string(),
				Token::ModulusAssign => "%=".to_string(),
				Token::PowerAssign => "**=".to_string(),
				Token::AndAssign => "&&=".to_string(),
				Token::OrAssign => "||=".to_string(),
				Token::BitAndAssign => "&=".to_string(),
				Token::BitOrAssign => "|=".to_string(),
				Token::BitXorAssign => "^=".to_string(),
				Token::LeftShiftAssign => "<<=".to_string(),
				Token::RightShiftAssign => ">>=".to_string(),
				Token::Arrow => "->".to_string(),
				Token::Increment => "++".to_string(),
				Token::Decrement => "--".to_string(),
				Token::LeftParen => "(".to_string(),
				Token::RightParen => ")".to_string(),
				Token::LeftBracket => "[".to_string(),
				Token::RightBracket => "]".to_string(),
				Token::LeftBrace => "{".to_string(),
				Token::RightBrace => "}".to_string(),
				Token::Exclamation => "!".to_string(),
				Token::QuestionMark => "?".to_string(),
				Token::Period => ".".to_string(),
				Token::Comma => ",".to_string(),
				Token::Backslash => "\\".to_string(),
				Token::AtSign => "@".to_string(),
				Token::Hash => "#".to_string(),
				Token::DollarSign => "$".to_string(),
				Token::Underscore => "_".to_string(),
				Token::EqualSign => "=".to_string(),
				Token::Apostrophe => "'".to_string(),
				Token::Quote => "\"".to_string(),
				Token::Tilde => "~".to_string(),
				Token::Backtick => "`".to_string(),
				Token::Semicolon => ";".to_string(),
				Token::Colon => ":".to_string(),
			}
		)
	}
}

#[derive(Error, Default, Debug, Clone, PartialEq)]
pub(crate) enum LexerError {
	#[error("Invalid integer literal: {0}")]
	InvalidIntegerLiteral(ParseIntError),
	#[error("Invalid float literal: {0}")]
	InvalidFloatLiteral(ParseFloatError),
	#[error("Invalid character literal: {0}")]
	InvalidCharLiteral(ParseCharError),
	#[error("Could not construct a valid character: {0}")]
	CharLiteralOverflow(CharTryFromError),
	#[default]
	#[error("Could not parse this token")]
	InvalidToken,
}

impl From<ParseIntError> for LexerError {
	fn from(err: ParseIntError) -> Self {
		LexerError::InvalidIntegerLiteral(err)
	}
}

impl From<ParseFloatError> for LexerError {
	fn from(err: ParseFloatError) -> Self {
		LexerError::InvalidFloatLiteral(err)
	}
}

impl From<ParseCharError> for LexerError {
	fn from(err: ParseCharError) -> Self {
		LexerError::InvalidCharLiteral(err)
	}
}

impl From<CharTryFromError> for LexerError {
	fn from(err: CharTryFromError) -> Self {
		LexerError::CharLiteralOverflow(err)
	}
}
