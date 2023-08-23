use inkwell::{builder::Builder, context::Context, module::Module};
use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;

use crate::parser::ast::ZephyrModule;
use crate::parser::Rule::char;

#[must_use]
struct Compiler<'ctx> {
	context: &'ctx Context,
	module: Module<'ctx>,
	builder: Builder<'ctx>,
}

impl<'ctx> Compiler<'ctx> {
	fn new(context: &'ctx Context, module_name: &str) -> Self {
		Compiler {
			context,
			module: context.create_module(module_name),
			builder: context.create_builder(),
		}
	}

	fn new_anon(context: &'ctx Context) -> Self {
		Compiler::new(context, Compiler::random_module_name().as_str())
	}

	fn compile(&self, module: ZephyrModule) {}

	fn random_module_name() -> String {
		thread_rng()
			.sample_iter(&Alphanumeric)
			.take(10)
			.map(char::from)
			.collect()
	}
}
