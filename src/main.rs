use std::io::BufRead;

use color_eyre::eyre;
use pest::Parser;

use crate::parser::ZephyrParser;

mod compiler;
mod parser;

fn main() -> eyre::Result<()> {
	color_eyre::install()?;

	match ZephyrParser::parse_module("\
let x = 1;
let y = 2;\
") {
		Ok(module) => println!("{:#?}", module),
		Err(e) => println!("{}", e)
	}

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
			Err(e) => eprintln!("Parse failed: {}", e.renamed_rules(Rule::to_string)),
		}
	}*/

	Ok(())
}
