use std::error::Error;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ZephyrError<'a> {
	#[error("No identifier named {0} was found.")]
	UnknownIdent(&'a str),
	#[error("An error occurred during parsing: {0}")]
	ParseError(Box<dyn Error>)
}
