#![feature(let_chains)]

use std::io::{self, BufRead, Write};

use color_eyre::eyre;
use lalrpop_util::ParseError;
use tracing::Level;

use crate::parser::{grammar::ExprParser, lexer::Lexer, token::Token};

mod compiler;
mod error;
mod parser;
mod utils;

fn main() -> eyre::Result<()> {
	color_eyre::install()?;
	tracing_subscriber::fmt()
		.compact()
		.without_time()
		.with_max_level(Level::TRACE)
		.init();

	print!("> ");
	io::stdout().flush()?;
	for source in io::stdin().lock().lines().map_while(Result::ok) {
		if source == ":q" {
			break;
		}

		let lexer: Lexer<Token> = Lexer::new(&source[..]);
		println!("Lexer output: ");
		for token in lexer.clone() {
			println!("{:?}", token);
		}

		match ExprParser::new().parse(lexer) {
			Ok(expr) => println!("{:?}", expr),
			Err(err) => match err {
				ParseError::User { error } => println!("{}", error),
				_ => println!("{}", err),
			},
		}
		print!("> ");
		io::stdout().flush()?;
	}

	// match ZephyrParser::parse_module(program) {
	// 	Ok(module) => {
	// 		for (i, member) in Compiler::default().compile(&module).iter().enumerate() {
	// 			match member {
	// 				Ok(line) => println!("{}", line),
	// 				Err(err) => eprintln!(
	// 					"{} {}: {}",
	// 					"error".bold().bright_red(),
	// 					format!("on line {}", (i + 1).bright_white()).dimmed(),
	// 					err.bold()
	// 				),
	// 			}
	// 		}
	// 	}
	// 	Err(err) => eprintln!("{}", err.bright_red()),
	// }

	/*for line in io::stdin().lock().lines() {
		match ZephyrParser::parse(Rule::single_expr, &line?) {
			Ok(mut pairs) => {
				println!(
					"Parsed: {:#?}",
					ZephyrParser::parse_expr(
						pairs
							.next()
							.unwrap()
							.into_inner() // gets the expr inside single_expr
							.next()
							.unwrap()
							.into_inner() // gets the pairs inside the expr
					)
				);
			}
			Err(e) => error!("Parse failed: {}", e.renamed_rules(Rule::to_string)),
		}
	}*/

	Ok(())
}
