use std::collections::{BTreeMap, HashMap};
use std::fmt::{Debug, Display, Formatter};
use std::hash::Hash;

use itertools::Itertools;
use ordered_float::OrderedFloat;

use crate::parser::Rule;

pub enum ZephyrValue<'a> {
	Int(i64),
	Float(f64),
	Char(char),
	String(&'a str),
	Boolean(bool),
	List(Vec<ZephyrValue<'a>>),
	Map(HashMap<ZephyrValue<'a>, ZephyrValue<'a>>),
	Tuple(Vec<ZephyrValue<'a>>),
	Object(HashMap<&'a str, ZephyrValue<'a>>),
	Void,
	Closure {
		params: HashMap<&'a str, ZephyrValue<'a>>,
		body: ZephyrExpression<'a>,
		return_type: ZephyrType<'a>,
	},
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum ZephyrExpression<'a> {
	Int(i64),
	Float(OrderedFloat<f64>),
	Char(char),
	String(String),
	Boolean(bool),
	List(Vec<ZephyrExpression<'a>>),
	Map(BTreeMap<ZephyrExpression<'a>, ZephyrExpression<'a>>),
	Tuple(Vec<ZephyrExpression<'a>>),
	Object(BTreeMap<&'a str, ObjectData<'a>>),
	Closure {
		params: BTreeMap<&'a str, ZephyrType<'a>>,
		return_type: Option<ZephyrType<'a>>,
		body: Box<ZephyrExpression<'a>>,
	},
	Ident(&'a str),
	Call {
		callee: Box<ZephyrExpression<'a>>,
		args: Vec<ZephyrExpression<'a>>,
	},
	Assignment {
		ident: &'a str,
		new: Box<ZephyrExpression<'a>>,
		op: AssignOp,
	},
	BinaryOp {
		op: BinaryOp,
		lhs: Box<ZephyrExpression<'a>>,
		rhs: Box<ZephyrExpression<'a>>,
	},
	PrefixOp {
		op: PrefixOp,
		expr: Box<ZephyrExpression<'a>>,
	},
	PostfixOp {
		op: PostfixOp,
		expr: Box<ZephyrExpression<'a>>,
	},
	Let {
		name: &'a str,
		mutable: bool,
		value: Box<ZephyrExpression<'a>>,
		type_def: Option<ZephyrType<'a>>,
	},
	If {
		condition: Box<ZephyrExpression<'a>>,
		then: Box<ZephyrExpression<'a>>,
		otherwise: Option<Box<ZephyrExpression<'a>>>,
	},
	While {
		condition: Box<ZephyrExpression<'a>>,
		body: Box<ZephyrExpression<'a>>,
	},
	For {
		name: &'a str,
		iterable: Box<ZephyrExpression<'a>>,
		body: Box<ZephyrExpression<'a>>,
	},
	Import {
		path: String,
		idents: Option<BTreeMap<&'a str, Option<&'a str>>>,
	},
	Cast {
		expr: Box<ZephyrExpression<'a>>,
		to_type: ZephyrType<'a>,
	},
	TypeCheck {
		expr: Box<ZephyrExpression<'a>>,
		is_type: ZephyrType<'a>,
	},
	CodeBlock(Vec<ZephyrExpression<'a>>),
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct ObjectData<'a> {
	pub(crate) value: ZephyrExpression<'a>,
	pub(crate) mutable: bool,
	pub(crate) type_def: Option<ZephyrType<'a>>,
}

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
			}
		)
	}
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum ZephyrType<'a> {
	Int,
	Float,
	Char,
	String,
	Boolean,
	Void,
	List(Box<ZephyrType<'a>>),
	Map(Box<ZephyrType<'a>>, Box<ZephyrType<'a>>),
	Tuple(Vec<ZephyrType<'a>>),
	Object(BTreeMap<&'a str, Box<ZephyrType<'a>>>),
	Closure {
		params: Vec<ZephyrType<'a>>,
		return_type: Box<ZephyrType<'a>>,
	},
	Union(Vec<ZephyrType<'a>>),
}

impl Display for ZephyrType<'_> {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}",
			match self {
				ZephyrType::Int => String::from("int"),
				ZephyrType::Float => String::from("float"),
				ZephyrType::Char => String::from("char"),
				ZephyrType::String => String::from("string"),
				ZephyrType::Boolean => String::from("boolean"),
				ZephyrType::Void => String::from("void"),
				ZephyrType::List(elem) => format!("#[{}]", elem),
				ZephyrType::Map(key, value) => format!("#{{{}: {}}}", key, value),
				ZephyrType::Tuple(items) =>
					format!("#({})", items.iter().map(ZephyrType::to_string).join(", ")),
				ZephyrType::Object(fields) => format!(
					"@{{{}}}",
					fields
						.into_iter()
						.map(|(k, v)| format!("{}: {}", k, v))
						.join(", ")
				),
				ZephyrType::Closure {
					params,
					return_type,
				} => format!(
					"({}) -> {}",
					params.iter().map(ZephyrType::to_string).join(", "),
					return_type
				),
				ZephyrType::Union(parts) => parts.iter().join(" | "),
			}
		)
	}
}

// this turns all the rules into human friendly names to be used in
// error messages
impl ToString for Rule {
	fn to_string(&self) -> String {
		String::from(match self {
			Rule::EOI => "end of input",
			Rule::program => "a program",
			Rule::WHITESPACE => "whitespace",
			Rule::COMMENT => "a comment",
			Rule::expr => "an expression",
			Rule::single_expr => "a single expression",
			Rule::primary => "an expression",
			Rule::float => "a floating point number",
			Rule::integer => "an integer",
			Rule::decimal_int => "a decimal integer",
			Rule::binary_int => "a binary integer",
			Rule::octal_int => "an octal integer",
			Rule::hex_int => "a hexadecimal integer",
			Rule::exponent => "an integer with an exponent",
			Rule::string => "a string literal",
			Rule::char => "a character literal",
			Rule::inner_string => "the contents of a string literal",
			Rule::text_literal_char => "a part of a text literal",
			Rule::boolean => "a boolean",
			Rule::true_bool => "`true`",
			Rule::false_bool => "`false`",
			Rule::list => "a list expression",
			Rule::map => "a map expression",
			Rule::map_entry => "a map entry",
			Rule::tuple => "a tuple expression",
			Rule::object => "an object expression",
			Rule::object_entry => "an object field",
			Rule::closure => "a closure",
			Rule::closure_param => "a closure parameter",
			Rule::call => "a closure call",
			Rule::let_expr => "a `let` declaration",
			Rule::mutable => "`mut`",
			Rule::if_expr => "an `if` expression",
			Rule::while_expr => "a `while` loop",
			Rule::for_expr => "a `for` loop",
			Rule::import => "an import call",
			Rule::import_idents_clause => "a list of imported identifiers",
			Rule::import_ident => "an imported identifier",
			Rule::cast => "a cast expression",
			Rule::type_check => "a type check expression",
			Rule::code_block => "a code block",
			Rule::term => "an expression",
			Rule::parens => "a parenthesized expression",
			Rule::assignment => "an assignment expression",
			Rule::assign_op => "an assignment operator",
			Rule::assign => "=",
			Rule::plus_assign => "+=",
			Rule::minus_assign => "-=",
			Rule::times_assign => "*=",
			Rule::divide_assign => "/=",
			Rule::modulus_assign => "%=",
			Rule::power_assign => "**=",
			Rule::and_assign => "&&=",
			Rule::or_assign => "||=",
			Rule::type_def => "a type definition",
			Rule::type_primary => "a type definition",
			Rule::int_type => "an `int` type",
			Rule::float_type => "a `float` type",
			Rule::string_type => "a `string` type",
			Rule::char_type => "a `char` type",
			Rule::boolean_type => "a `boolean` type",
			Rule::void_type => "a `void` type",
			Rule::list_type => "a list type",
			Rule::map_type => "a map type",
			Rule::tuple_type => "a tuple type",
			Rule::object_type => "an object type",
			Rule::object_type_entry => "an object type's field",
			Rule::closure_type => "a closure type",
			Rule::infix => "an infix operator",
			Rule::in_op => "in",
			Rule::not_in => "!in",
			Rule::not_equals => "!=",
			Rule::equals => "==",
			Rule::plus => "+",
			Rule::minus => "-",
			Rule::times => "*",
			Rule::divide => "/",
			Rule::modulus => "%",
			Rule::power => "**",
			Rule::less_equals => "<=",
			Rule::less_than => "<",
			Rule::greater_equals => ">=",
			Rule::greater_than => ">",
			Rule::and => "&&",
			Rule::or => "||",
			Rule::prefix => "a prefix operator",
			Rule::not => "!",
			Rule::negate => "-",
			Rule::postfix => "a postfix operator",
			Rule::increment => "++",
			Rule::decrement => "--",
			Rule::identifier => "an identifier",
			Rule::reserved_name => "a reserved name",
		})
	}
}
