use std::{collections::BTreeMap, fmt::Debug, hash::Hash};

use derive_builder::Builder;
use ordered_float::OrderedFloat;

use crate::parser::ast::{
	operators::{AssignOp, BinaryOp, PostfixOp, PrefixOp},
	structs::FieldData,
	types::{GenericConstraint, TypeDef},
};

pub(crate) mod operators;
pub(crate) mod structs;
pub(crate) mod types;

#[derive(Debug, Clone, PartialEq, PartialOrd, Builder)]
pub struct ZephyrModule<'ctx> {
	pub(crate) members: BTreeMap<&'ctx str, Declaration<'ctx>>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum Declaration<'ctx> {
	Variable(VariableData<'ctx>),
	Struct {
		fields: BTreeMap<&'ctx str, FieldData<'ctx>>,
		/// Maps each generic type param to its constraints
		type_params: BTreeMap<&'ctx str, Option<GenericConstraint>>,
	},
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum Expr<'ctx> {
	Int(u64), // this can be unsigned since negatives are represented with a prefix op expr
	Float(OrderedFloat<f64>),
	Boolean(bool),
	Char(char),
	String(Vec<StringComponent<'ctx>>),
	Ident(&'ctx str),
	List(Vec<Expr<'ctx>>),
	Map(BTreeMap<Expr<'ctx>, Expr<'ctx>>),
	Tuple(Vec<Expr<'ctx>>),
	Object(BTreeMap<&'ctx str, ObjectData<'ctx>>),
	Closure {
		params: BTreeMap<&'ctx str, TypeDef<'ctx>>,
		return_type: Option<TypeDef<'ctx>>,
		body: Box<Expr<'ctx>>,
	},
	Call {
		callee: Box<Expr<'ctx>>,
		args: Vec<(Option<&'ctx str>, Expr<'ctx>)>,
	},
	MemberAccess {
		parent: Box<Expr<'ctx>>,
		member: &'ctx str,
	},
	IndexAccess {
		parent: Box<Expr<'ctx>>,
		index: Box<Expr<'ctx>>,
	},
	Assignment {
		ident: &'ctx str,
		new: Box<Expr<'ctx>>,
		op: AssignOp,
	},
	BinaryOp {
		op: BinaryOp,
		lhs: Box<Expr<'ctx>>,
		rhs: Box<Expr<'ctx>>,
	},
	PrefixOp {
		op: PrefixOp,
		expr: Box<Expr<'ctx>>,
	},
	PostfixOp {
		op: PostfixOp,
		expr: Box<Expr<'ctx>>,
	},
	Let(&'ctx str, VariableData<'ctx>),
	If {
		condition: Box<Expr<'ctx>>,
		then: Box<Expr<'ctx>>,
		otherwise: Option<Box<Expr<'ctx>>>,
	},
	While {
		condition: Box<Expr<'ctx>>,
		body: Box<Expr<'ctx>>,
	},
	For {
		name: &'ctx str,
		iterable: Box<Expr<'ctx>>,
		body: Box<Expr<'ctx>>,
	},
	Import {
		path: String,
		idents: Option<BTreeMap<&'ctx str, Option<&'ctx str>>>,
	},
	Cast {
		expr: Box<Expr<'ctx>>,
		to_type: TypeDef<'ctx>,
	},
	TypeCheck {
		expr: Box<Expr<'ctx>>,
		is_type: TypeDef<'ctx>,
		negated: bool
	},
	Return(Box<Expr<'ctx>>),
	CodeBlock(Vec<Expr<'ctx>>),
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) enum StringComponent<'ctx> {
	Grapheme(char),
	ShortEscape(ShortEscapeType),
	UnicodeEscape(char),
	InterpolatedExpr(Expr<'ctx>),
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub(crate) enum ShortEscapeType {
	Quote,
	CarriageReturn,
	Newline,
	Backslash,
	Tab,
	Dollar,
}

impl ShortEscapeType {
	pub(crate) fn character(&self) -> char {
		match self {
			ShortEscapeType::Quote => '"',
			ShortEscapeType::CarriageReturn => '\r',
			ShortEscapeType::Newline => '\n',
			ShortEscapeType::Backslash => '\\',
			ShortEscapeType::Tab => '\t',
			ShortEscapeType::Dollar => '$',
		}
	}
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd, Builder)]
pub struct ObjectData<'ctx> {
	pub(crate) value: Expr<'ctx>,
	pub(crate) mutable: bool,
	pub(crate) type_def: Option<TypeDef<'ctx>>,
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Builder)]
pub struct VariableData<'ctx> {
	pub(crate) mutable: bool,
	pub(crate) value: Box<Expr<'ctx>>,
	pub(crate) type_def: Option<TypeDef<'ctx>>,
}
