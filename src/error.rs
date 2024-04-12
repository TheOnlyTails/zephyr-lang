use crate::parser::ast::{
	operators::{AssignOp, BinaryOp, PostfixOp, PrefixOp},
	types::TypeDef,
};
use itertools::Itertools;
use std::{
	error::Error,
	fmt::{Debug, Display, Formatter},
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ZephyrError<'ctx> {
	#[error(r#"No identifier named "{0}" was found."#)]
	UnknownIdent(&'ctx str),
	#[error(r#"No field named "{0}" exists on this object. Try one of the following: {}"#, .1.iter().map(|s| format!(r#""{s}""#)).join(", "))]
	NoSuchField(&'ctx str, Vec<&'ctx str>),
	#[error("Type {0} is not assignable to {1}")]
	UnassignableTypes(TypeDef<'ctx>, TypeDef<'ctx>),
	#[error("Values of type {0} are not iterables")]
	NotAnIterable(TypeDef<'ctx>),
	#[error("Cannot call a value of type {0} since it is not a closure")]
	NotAClosure(TypeDef<'ctx>),
	#[error("Cannot perform member access on a value whose type is not an object: {0}")]
	MemberAccessOnNonObject(TypeDef<'ctx>),
	#[error("Cannot index into a value of type {0}")]
	UnindexableValue(TypeDef<'ctx>),
	#[error("The type {1} cannot be used to index a value of type {0}, expected {2}")]
	WrongIndex(TypeDef<'ctx>, TypeDef<'ctx>, TypeDef<'ctx>),
	#[error("Cannot access a collection of size {1} at index {0}, will throw an error at runtime")]
	IndexOutOfBounds(i64, usize),
	#[error("Cannot use negative indexing on tuple types.")]
	NegativeIndexOnTuples,
	#[error("The type of the provided argument ({0}) does not match the type of the expected closure parameter")]
	WrongArgType(TypeDef<'ctx>, TypeDef<'ctx>),
	#[error("Cannot apply the {0} binary operator to values of type {1} and {2}")]
	InapplicableBinaryOp(BinaryOp, TypeDef<'ctx>, TypeDef<'ctx>),
	#[error("Cannot apply the {0} prefix operator to a value of type {1}")]
	InapplicablePrefixOp(PrefixOp, TypeDef<'ctx>),
	#[error("Cannot apply the {0} postfix operator to a value of type {1}")]
	InapplicablePostfixOp(PostfixOp, TypeDef<'ctx>),
	#[error(
		"Cannot use the {0} assignment operation with a value of type {2} assigned to one of type {1}"
	)]
	InapplicableAssignmentOp(AssignOp, TypeDef<'ctx>, TypeDef<'ctx>),
	#[error("The immutable variable `{0}` may not be reassigned.")]
	NoImmutableAssignments(&'ctx str),
	#[error("The type of the declaration `{0}` cannot be inferred from its value. Please add a type declaration or explicitly cast the value to the desired type.")]
	CannotInferType(&'ctx str),
	#[error("An error occurred during parsing: {0}")]
	ParseError(Box<dyn Error>),
}

#[derive(Debug)]
pub enum ZephyrWarning {
	NoIsVoid,
}

impl Display for ZephyrWarning {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}",
			match self {
				ZephyrWarning::NoIsVoid => "Performing a type-check against the `void` type always fails.",
			}
		)
	}
}
