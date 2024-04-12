use lalrpop_util::lalrpop_mod;

pub(crate) mod lexer;
pub(crate) mod string;
pub(crate) mod token;
pub(crate) mod ast;

lalrpop_mod!(pub(crate) grammar, "/parser/grammar.rs");

mod test {
	use crate::parser::{grammar::ExprParser, lexer::Lexer};

	#[test]
	fn grammar() {
		assert!(ExprParser::new().parse(Lexer::new("0x1aF42")).is_ok());
	}
}
