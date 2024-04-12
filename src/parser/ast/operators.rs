use std::fmt::{Display, Formatter};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum BinaryOp {
	Plus,
	Minus,
	Times,
	Divide,
	Modulus,
	Power,
	Equals,
	NotEquals,
	LessThan,
	LessThanEquals,
	GreaterThan,
	GreaterThanEquals,
	And,
	Or,
	In,
	NotIn,
	BitAnd,
	BitOr,
	BitXor,
	LeftShift,
	RightShift,
	RangeTo,
	RangeToEquals,
}

impl BinaryOp {
	pub(crate) const NUM_OPS: [Self; 5] = [
		BinaryOp::Minus,
		BinaryOp::Times,
		BinaryOp::Divide,
		BinaryOp::Modulus,
		BinaryOp::Power,
	];
	pub(crate) const EQ_OPS: [Self; 2] = [BinaryOp::Equals, BinaryOp::NotEquals];
	pub(crate) const COMP_OPS: [Self; 4] = [
		BinaryOp::LessThan,
		BinaryOp::LessThanEquals,
		BinaryOp::GreaterThan,
		BinaryOp::GreaterThanEquals,
	];
	pub(crate) const BOOL_OPS: [Self; 2] = [BinaryOp::And, BinaryOp::Or];
}

impl Display for BinaryOp {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}",
			match self {
				BinaryOp::Plus => "+",
				BinaryOp::Minus => "-",
				BinaryOp::Times => "*",
				BinaryOp::Divide => "/",
				BinaryOp::Modulus => "%",
				BinaryOp::Power => "**",
				BinaryOp::Equals => "==",
				BinaryOp::NotEquals => "!=",
				BinaryOp::LessThan => "<",
				BinaryOp::LessThanEquals => "<=",
				BinaryOp::GreaterThan => ">",
				BinaryOp::GreaterThanEquals => ">=",
				BinaryOp::And => "&&",
				BinaryOp::Or => "||",
				BinaryOp::In => "in",
				BinaryOp::NotIn => "!in",
				BinaryOp::BitAnd => "&",
				BinaryOp::BitOr => "|",
				BinaryOp::BitXor => "^",
				BinaryOp::LeftShift => "<<",
				BinaryOp::RightShift => ">>",
				BinaryOp::RangeTo => "..",
				BinaryOp::RangeToEquals => "..=",
			}
		)
	}
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum PrefixOp {
	Not,
	Negate,
}

impl Display for PrefixOp {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}",
			match self {
				PrefixOp::Not => "!",
				PrefixOp::Negate => "-",
			}
		)
	}
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum PostfixOp {
	Increment,
	Decrement,
}

impl PostfixOp {
	pub(crate) const NUM_OPS: [Self; 2] = [PostfixOp::Increment, PostfixOp::Decrement];
}

impl Display for PostfixOp {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}",
			match self {
				PostfixOp::Increment => "++",
				PostfixOp::Decrement => "--",
			}
		)
	}
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum AssignOp {
	None,
	Plus,
	Minus,
	Times,
	Divide,
	Modulus,
	Power,
	And,
	Or,
	BitAnd,
	BitOr,
	BitXor,
	LeftShift,
	RightShift,
}

impl AssignOp {
	pub(crate) const NUM_OPS: [Self; 5] = [
		AssignOp::Minus,
		AssignOp::Times,
		AssignOp::Divide,
		AssignOp::Modulus,
		AssignOp::Power,
	];

	pub(crate) const BOOL_OPS: [Self; 2] = [AssignOp::And, AssignOp::Or];
}

impl Display for AssignOp {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}",
			match self {
				AssignOp::None => "=",
				AssignOp::Plus => "+=",
				AssignOp::Minus => "-=",
				AssignOp::Times => "*=",
				AssignOp::Divide => "/=",
				AssignOp::Modulus => "%=",
				AssignOp::Power => "**=",
				AssignOp::And => "&&=",
				AssignOp::Or => "||=",
				AssignOp::BitAnd => "&=",
				AssignOp::BitOr => "|=",
				AssignOp::BitXor => "^=",
				AssignOp::LeftShift => "<<=",
				AssignOp::RightShift => ">>=",
			}
		)
	}
}
