use crate::parser::ast::{types::TypeDef, Expr};
use derive_builder::Builder;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd, Builder)]
pub(crate) struct FieldData<'ctx> {
	#[builder(default)]
	pub(crate) visibility: Visibility,
	#[builder(default = "false")]
	pub(crate) mutable: bool,
	pub(crate) type_def: TypeDef<'ctx>,
	#[builder(default)]
	pub(crate) default: Option<Expr<'ctx>>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub(crate) enum Visibility {
	Public,
	Internal,
	Private,
}

impl Default for Visibility {
	fn default() -> Self {
		Visibility::Internal
	}
}
