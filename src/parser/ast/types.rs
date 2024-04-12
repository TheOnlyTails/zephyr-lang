use itertools::Itertools;
use std::{
	collections::{BTreeMap, BTreeSet},
	fmt::{Display, Formatter},
	ops::Deref,
};

#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum TypeDef<'ctx> {
	Int,
	Float,
	Char,
	String,
	Boolean,
	Void,
	List(Box<TypeDef<'ctx>>),
	Map(Box<TypeDef<'ctx>>, Box<TypeDef<'ctx>>),
	Tuple(Vec<TypeDef<'ctx>>),
	Object(BTreeMap<&'ctx str, Box<TypeDef<'ctx>>>),
	Closure {
		params: Vec<TypeDef<'ctx>>,
		return_type: Box<TypeDef<'ctx>>,
	},
	Union(BTreeSet<TypeDef<'ctx>>),
	Identifier {
		name: &'ctx str,
		generics: Vec<TypeDef<'ctx>>,
	},
}

impl TypeDef<'_> {
	fn simplify(&self) -> Self {
		match self {
			TypeDef::Int
			| TypeDef::Float
			| TypeDef::Char
			| TypeDef::String
			| TypeDef::Boolean
			| TypeDef::Void => self.clone(),
			TypeDef::List(type_def) => TypeDef::List(type_def.simplify().into()),
			TypeDef::Map(key, value) => TypeDef::Map(key.simplify().into(), value.simplify().into()),
			TypeDef::Tuple(items) => TypeDef::Tuple(items.iter().map(TypeDef::simplify).collect()),
			TypeDef::Object(fields) => TypeDef::Object(
				fields
					.iter()
					.map(|(&name, value)| (name, value.simplify().into()))
					.collect(),
			),
			TypeDef::Closure {
				params,
				return_type,
			} => TypeDef::Closure {
				params: params.iter().map(TypeDef::simplify).collect(),
				return_type: return_type.simplify().into(),
			},
			TypeDef::Union(_options) => {
				todo!()
			}
			TypeDef::Identifier { name, generics } => {
				todo!()
			}
		}
	}

	pub(crate) fn index_type(&self) -> Option<&Self> {
		match self {
			TypeDef::String | TypeDef::List(_) | TypeDef::Tuple(_) => Some(&TypeDef::Int),
			TypeDef::Map(key_type, _) => Some(&key_type.deref()),
			TypeDef::Object(_) => Some(&TypeDef::String),
			_ => None,
		}
	}

	pub(crate) fn assignable_to(&self, other: &Self) -> bool {
		// T <=> T
		if self == other {
			return true;
		}

		// int <=> float
		if matches!(
			(self, other),
			(TypeDef::Int, TypeDef::Float) | (TypeDef::Float, TypeDef::Int)
		) {
			return true;
		}

		// string <=> #[char]
		if let TypeDef::List(item) = self {
			if item.assignable_to(&TypeDef::Char) && other.assignable_to(&TypeDef::String) {
				return true;
			}
		}
		if let TypeDef::List(item) = other {
			if item.assignable_to(&TypeDef::Char) && self.assignable_to(&TypeDef::String) {
				return true;
			}
		}

		// A | B => A | B | C
		if let TypeDef::Union(self_set) = self {
			if let TypeDef::Union(other_set) = other {
				// all members of self_set are assignable to some member of other_set
				if self_set
					.iter()
					.all(|t| other_set.iter().any(|ot| t.assignable_to(ot)))
				{
					return true;
				}
			}
		}
		if let TypeDef::Union(self_set) = other {
			if let TypeDef::Union(other_set) = self {
				if self_set
					.iter()
					.all(|t| other_set.iter().any(|ot| t.assignable_to(ot)))
				{
					return true;
				}
			}
		}

		// A => A | B
		if let TypeDef::Union(set) = other {
			if set.iter().any(|t| t.assignable_to(self)) {
				return true;
			}
		}

		// #{string: T} <=> @{a: T, b: T}
		if let TypeDef::Map(key_type, value_type) = self {
			if let TypeDef::Object(fields) = other {
				// the map's key is a string and the value type is assignable to all the fields
				if key_type.assignable_to(&TypeDef::String)
					&& fields.values().all(|t| value_type.assignable_to(t))
				{
					return true;
				}
			}
		}

		if let TypeDef::Map(key_type, value_type) = other {
			if let TypeDef::Object(fields) = self {
				// the map's key is a string and the value type is assignable to all the fields
				if key_type.assignable_to(&TypeDef::String)
					&& fields.values().all(|t| t.assignable_to(value_type))
				{
					return true;
				}
			}
		}

		false
	}

	pub fn is_union<const N: usize>(&self, union: [Self; N]) -> bool {
		self.assignable_to(&TypeDef::Union(BTreeSet::from(union)))
	}

	pub fn is_number(&self) -> bool {
		self.is_union([TypeDef::Int, TypeDef::Float])
	}

	pub fn is_ordered(&self) -> bool {
		self.is_union([TypeDef::Int, TypeDef::Float, TypeDef::Char])
	}
}

impl Display for TypeDef<'_> {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}",
			match self {
				TypeDef::Int => String::from("int"),
				TypeDef::Float => String::from("float"),
				TypeDef::Char => String::from("char"),
				TypeDef::String => String::from("string"),
				TypeDef::Boolean => String::from("boolean"),
				TypeDef::Void => String::from("void"),
				TypeDef::List(elem) => format!("#[{}]", elem),
				TypeDef::Map(key, value) => format!("#{{{}: {}}}", key, value),
				TypeDef::Tuple(items) => format!("#({})", items.iter().map(TypeDef::to_string).join(", ")),
				TypeDef::Object(fields) => format!(
					"{{|{}|}}",
					fields
						.iter()
						.map(|(k, v)| format!("{}: {}", k, v))
						.join(", ")
				),
				TypeDef::Closure {
					params,
					return_type,
				} => format!(
					"({}) -> {}",
					params.iter().map(TypeDef::to_string).join(", "),
					return_type
				),
				TypeDef::Union(parts) => parts.iter().join(" | "),
				TypeDef::Identifier { name, generics } => {
					format!(
						"{name}{}",
						if generics.is_empty() {
							"".into()
						} else {
							format!("<{}>", generics.iter().join(", "))
						}
					)
				}
			}
		)
	}
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub(crate) struct GenericConstraint {

}
