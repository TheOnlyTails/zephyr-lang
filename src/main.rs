use color_eyre::eyre;
use std::io;
use std::io::BufRead;

use pest::Parser;

use crate::parser::{Rule, ZephyrParser};

mod parser;

fn main() -> eyre::Result<()> {
	color_eyre::install()?;

	for line in io::stdin().lock().lines() {
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
			Err(e) => eprintln!("Parse failed: {}", e.renamed_rules(Rule::to_string)),
		}
	}
	Ok(())
}
