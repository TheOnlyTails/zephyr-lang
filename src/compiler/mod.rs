// use std::{
// 	borrow::Cow,
// 	collections::{BTreeMap, BTreeSet},
// };
//
// use color_eyre::Result;
// use itertools::Itertools;
//
// use crate::{
// 	error::ZephyrError,
// 	parser::ast::{
// 		AssignOp, BinaryOp, Expr, ObjectData, PostfixOp, PrefixOp, VarData, ZephyrModule, TypeDef,
// 	},
// };
//
// #[derive(Debug, Clone)]
// struct EnvData<'ctx> {
// 	mutable: bool,
// 	type_def: TypeDef<'ctx>,
// }
// type CompilerEnv<'ctx> = BTreeMap<&'ctx str, EnvData<'ctx>>;
//
// pub struct Compiler<'ctx> {
// 	top_level: Cow<'ctx, CompilerEnv<'ctx>>,
// }
// type CompileResult<'ctx> = Result<String, ZephyrError<'ctx>>;
// type ModuleResult<'ctx> = Vec<CompileResult<'ctx>>;
//
// impl<'ctx> Compiler<'ctx> {
// 	pub fn new(top_level: CompilerEnv<'ctx>) -> Self {
// 		Self {
// 			top_level: Cow::Owned(top_level),
// 		}
// 	}
//
// 	pub fn compile(&mut self, module: &'ctx ZephyrModule<'ctx>) -> ModuleResult<'ctx> {
// 		module
// 			.members
// 			.iter()
// 			.map(|data| self.compile_top_level(data, Expr::Let(data.0, data.1.clone())))
// 			.collect_vec()
// 	}
//
// 	fn compile_top_level(
// 		&mut self,
// 		expr: (&&'ctx str, &'ctx VarData<'ctx>),
// 		let_decl: Expr<'ctx>,
// 	) -> CompileResult<'ctx> {
// 		self.top_level.to_mut().insert(
// 			expr.0,
// 			EnvData {
// 				mutable: expr.1.mutable,
// 				type_def: expr
// 					.1
// 					.type_def
// 					.clone()
// 					.ok_or(ZephyrError::CannotInferType(expr.0))?,
// 			},
// 		);
// 		let mut local_env = self.top_level.clone();
// 		self.compile_expr(&let_decl, &mut local_env)
// 	}
//
// 	fn compile_expr<'a>(
// 		&self,
// 		expr: &'a Expr<'a>,
// 		env: &mut Cow<CompilerEnv<'a>>,
// 	) -> CompileResult<'a> {
// 		match expr {
// 			Expr::Int(value) => Ok(value.to_string()),
// 			Expr::Float(value) => Ok(value.to_string()),
// 			Expr::Char(value) => Ok(format!("{{ value: '{}', '0char': undefined }}", value)),
// 			Expr::String(value) => Ok(format!("`{}`", value.replace('$', "\\$"))),
// 			Expr::Boolean(value) => Ok(value.to_string()),
// 			Expr::List(value) | Expr::Tuple(value) => Ok(format!(
// 				"[{}]",
// 				value
// 					.iter()
// 					.map(move |e| self.compile_expr(e, env))
// 					.try_collect::<String, String, _>()
// 					.iter()
// 					.join(",")
// 			)),
// 			Expr::Map(value) => Ok(format!(
// 				"new Map([{}])",
// 				value
// 					.iter()
// 					.map(|(k, v)| Ok(format!(
// 						"[{},{}]",
// 						self.compile_expr(k, env)?,
// 						self.compile_expr(v, env)?
// 					)))
// 					.collect::<Result<Vec<String>, ZephyrError>>()?
// 					.iter()
// 					.join(",")
// 			)),
// 			// while defining objects with an IIFE seems weird, this actually makes sense
// 			// doing it this way, using `Object.defineProperty`, allows for runtime control over field mutability
// 			Expr::Object(value) => Ok(format!(
// 				"(() => {{ let obj={{}};{}return obj; }})()",
// 				value
// 					.iter()
// 					.map(|(name, ObjectData { value, mutable, .. })| Ok(format!(
// 						"Object.defineProperty(obj,'{name}',{{ value:{}, writable:{mutable}, enumerable:true }});",
// 						self.compile_expr(value, env)?
// 					)))
// 					.collect::<Result<String, ZephyrError>>()
// 					.iter()
// 					.join(",")
// 			)),
// 			Expr::Closure { params, body, .. } => Ok(format!(
// 				"({}) => {}",
// 				params.keys().join(","),
// 				self.compile_expr(body, {
// 					env.to_mut().extend(params.iter().map(|(&name, type_def)| {
// 						(
// 							name,
// 							EnvData {
// 								mutable: false,
// 								type_def: type_def.clone(),
// 							},
// 						)
// 					}));
// 					env
// 				})?
// 			)),
// 			Expr::Ident(name) => {
// 				if env.contains_key(name) {
// 					Ok(name.to_string())
// 				} else {
// 					Err(ZephyrError::UnknownIdent(name))
// 				}
// 			}
// 			Expr::Call { callee, args } => {
// 				let callee_type = Self::type_of(callee, env)?;
//
// 				if let TypeDef::Closure { params, .. } = callee_type {
// 					let arg_types: Vec<_> = args.iter().map(|e| Self::type_of(e, env)).try_collect()?;
//
// 					for (arg_type, param_type) in arg_types.iter().zip(params) {
// 						if !arg_type.assignable_to(&param_type) {
// 							return Err(ZephyrError::WrongArgType(arg_type.clone(), param_type));
// 						}
// 					}
//
// 					Ok(format!(
// 						"({})({})",
// 						self.compile_expr(callee, env)?,
// 						args
// 							.iter()
// 							.map(|e| self.compile_expr(e, env))
// 							.try_collect::<String, String, _>()
// 							.iter()
// 							.join(",")
// 					))
// 				} else {
// 					Err(ZephyrError::NotAClosure(callee_type))
// 				}
// 			}
// 			Expr::MemberAccess { parent, member } => {
// 				let parent_type = Self::type_of(parent, env)?;
// 				if let TypeDef::Object(fields) = parent_type {
// 					if fields.contains_key(member) {
// 						Ok(format!("({})['{member}']", self.compile_expr(parent, env)?))
// 					} else {
// 						Err(ZephyrError::NoSuchField(
// 							member,
// 							fields.keys().cloned().collect_vec(),
// 						))
// 					}
// 				} else {
// 					Err(ZephyrError::MemberAccessOnNonObject(parent_type))
// 				}
// 			}
// 			Expr::IndexAccess { parent, index } => {
// 				let parent_type = Self::type_of(parent, env)?;
//
// 				let expected_index_type = parent_type
// 					.index_type()
// 					.ok_or(ZephyrError::UnindexableValue(parent_type.clone()))?;
//
// 				let index_type = Self::type_of(parent, env)?;
// 				if index_type.assignable_to(expected_index_type) {
// 					Ok(format!(
// 						"({})[{}]",
// 						self.compile_expr(parent, env)?,
// 						self.compile_expr(index, env)?
// 					))
// 				} else {
// 					let expected_index = expected_index_type.clone();
// 					Err(ZephyrError::WrongIndex(
// 						parent_type,
// 						index_type,
// 						expected_index,
// 					))
// 				}
// 			}
// 			Expr::Assignment { ident, op, new } => {
// 				if env.contains_key(ident) {
// 					if !&env[ident].mutable {
// 						return Err(ZephyrError::NoImmutableAssignments(ident));
// 					}
//
// 					let ident_type = &env[ident].type_def;
// 					let new_type = Self::type_of(new, env)?;
//
// 					let none_cond = *op == AssignOp::None && new_type.assignable_to(ident_type);
// 					let plus_cond = *op == AssignOp::Plus
// 						&& ((ident_type.is_number() && new_type.is_number())
// 							|| (ident_type.assignable_to(&TypeDef::String)
// 								&& new_type.assignable_to(&TypeDef::String)));
// 					let num_cond =
// 						AssignOp::NUM_OPS.contains(op) && ident_type.is_number() && new_type.is_number();
// 					let bool_cond = AssignOp::BOOL_OPS.contains(op)
// 						&& ident_type.assignable_to(&TypeDef::Boolean)
// 						&& new_type.assignable_to(&TypeDef::Boolean);
//
// 					if none_cond || plus_cond || num_cond || bool_cond {
// 						Ok(format!("{ident} {op} {}", self.compile_expr(new, env)?))
// 					} else {
// 						Err(ZephyrError::InapplicableAssignmentOp(
// 							*op,
// 							ident_type.clone(),
// 							new_type,
// 						))
// 					}
// 				} else {
// 					Err(ZephyrError::UnknownIdent(ident))
// 				}
// 			}
// 			Expr::BinaryOp { lhs, op, rhs } => {
// 				let lhs_type = Self::type_of(lhs, env)?;
// 				let rhs_type = Self::type_of(rhs, env)?;
// 				if let BinaryOp::In | BinaryOp::NotIn = op {
// 					Ok(format!(
// 						"({}).{}({})",
// 						if matches!(
// 							rhs_type,
// 							TypeDef::String
// 								| TypeDef::List(_)
// 								| TypeDef::Map(_, _)
// 								| TypeDef::Tuple(_)
// 								| TypeDef::Object(_)
// 						) {
// 							self.compile_expr(rhs, env)?
// 						} else {
// 							Err(ZephyrError::NotAnIterable(rhs_type.clone()))?
// 						},
// 						match rhs_type {
// 							TypeDef::String | TypeDef::List(_) | TypeDef::Tuple(_) => "includes",
// 							TypeDef::Map(_, _) => "has",
// 							TypeDef::Object(_) => "hasOwnProperty",
// 							_ => Err(ZephyrError::NotAnIterable(rhs_type))?,
// 						},
// 						self.compile_expr(lhs, env)?
// 					))
// 				} else {
// 					let plus_cond = *op == BinaryOp::Plus
// 						&& ((lhs_type.is_number() && rhs_type.is_number())
// 							|| (lhs_type.assignable_to(&TypeDef::String)
// 								&& rhs_type.assignable_to(&TypeDef::String)));
//
// 					let num_op_cond =
// 						BinaryOp::NUM_OPS.contains(op) && lhs_type.is_number() && rhs_type.is_number();
//
// 					let eq_op_cond = BinaryOp::EQ_OPS.contains(op) && rhs_type.assignable_to(&lhs_type);
//
// 					let comp_op_cond = BinaryOp::COMP_OPS.contains(op)
// 						&& lhs_type.is_ordered()
// 						&& rhs_type.is_ordered()
// 						&& rhs_type.assignable_to(&lhs_type);
//
// 					let bool_op_cond = BinaryOp::BOOL_OPS.contains(op)
// 						&& rhs_type.assignable_to(&TypeDef::Boolean)
// 						&& lhs_type.assignable_to(&TypeDef::Boolean);
//
// 					if plus_cond || num_op_cond || eq_op_cond || comp_op_cond || bool_op_cond {
// 						Ok(format!(
// 							"{} {op} {}",
// 							self.compile_expr(lhs, env)?,
// 							self.compile_expr(rhs, env)?
// 						))
// 					} else {
// 						Err(ZephyrError::InapplicableBinaryOp(*op, lhs_type, rhs_type))
// 					}
// 				}
// 			}
// 			Expr::PrefixOp { op, expr } => {
// 				let expr_type = Self::type_of(expr, env)?;
// 				if (*op == PrefixOp::Not && expr_type.assignable_to(&TypeDef::Boolean))
// 					|| (*op == PrefixOp::Negate && expr_type.is_number())
// 				{
// 					Ok(format!("{op}{}", self.compile_expr(expr, env)?))
// 				} else {
// 					Err(ZephyrError::InapplicablePrefixOp(*op, expr_type))
// 				}
// 			}
// 			Expr::PostfixOp { op, expr } => {
// 				let expr_type = Self::type_of(expr, env)?;
// 				if PostfixOp::NUM_OPS.contains(op) && expr_type.is_number() {
// 					Ok(format!("{}{op}", self.compile_expr(expr, env)?))
// 				} else {
// 					Err(ZephyrError::InapplicablePostfixOp(*op, expr_type))
// 				}
// 			}
// 			Expr::Let(
// 				name,
// 				VarData {
// 					mutable,
// 					value,
// 					type_def,
// 				},
// 			) => {
// 				let value_type = Self::type_of(value, env)?;
//
// 				if let Some(type_def) = type_def {
// 					if !value_type.assignable_to(type_def) {
// 						return Err(ZephyrError::UnassignableTypes(value_type, type_def.clone()));
// 					}
// 				}
//
// 				if name.clone() != "_" {
// 					env.to_mut().extend([(
// 						*name,
// 						EnvData {
// 							type_def: value_type,
// 							mutable: *mutable,
// 						},
// 					)]);
// 				}
//
// 				Ok(format!(
// 					"{} {name} = {}",
// 					if *mutable { "let" } else { "const" },
// 					self.compile_expr(value, env)?
// 				))
// 			}
// 			Expr::If {
// 				condition,
// 				then,
// 				otherwise,
// 			} => Ok(if let Some(otherwise) = otherwise {
// 				format!(
// 					"({}) ? ({}) : ({})",
// 					self.compile_expr(condition, env)?,
// 					self.compile_expr(then, env)?,
// 					self.compile_expr(otherwise, env)?
// 				)
// 			} else {
// 				format!(
// 					"if ({}) {{ {} }}",
// 					self.compile_expr(condition, env)?,
// 					self.compile_expr(then, env)?
// 				)
// 			}),
// 			Expr::While { condition, body } => Ok(format!(
// 				"while ({}) {{ {} }}",
// 				self.compile_expr(condition, env)?,
// 				self.compile_expr(body, env)?
// 			)),
// 			Expr::For {
// 				name,
// 				iterable,
// 				body,
// 			} => {
// 				let iterable_type = Self::type_of(iterable, env)?;
// 				if !matches!(
// 					iterable_type,
// 					TypeDef::String
// 						| TypeDef::List(_)
// 						| TypeDef::Map(_, _)
// 						| TypeDef::Tuple(_)
// 						| TypeDef::Object(_)
// 				) {
// 					return Err(ZephyrError::NotAnIterable(iterable_type));
// 				}
//
// 				let iterable_expr = self.compile_expr(iterable, env)?;
// 				Ok(format!(
// 					"for (let {name} of {}) {{ {} }}",
// 					if matches!(iterable_type, TypeDef::Object(_)) {
// 						format!("Object.entries({iterable_expr})")
// 					} else {
// 						iterable_expr
// 					},
// 					self.compile_expr(body, {
// 						env.to_mut().extend([(
// 							*name,
// 							EnvData {
// 								mutable: false,
// 								type_def: match iterable_type {
// 									TypeDef::String => TypeDef::Char,
// 									TypeDef::List(item) => *item,
// 									TypeDef::Map(k, v) => TypeDef::Tuple(vec![*k, *v]),
// 									TypeDef::Tuple(items) => {
// 										TypeDef::Union(items.iter().cloned().collect::<BTreeSet<_>>())
// 									}
// 									TypeDef::Object(fields) => TypeDef::Tuple(vec![
// 										TypeDef::String,
// 										TypeDef::Union(fields.values().map(|t| t.as_ref().clone()).collect()),
// 									]),
// 									_ => unreachable!(),
// 								},
// 							},
// 						)]);
// 						env
// 					})?
// 				))
// 			}
// 			Expr::Import { .. } => todo!(),
// 			Expr::Cast { expr, to_type } => {
// 				let expr_type = Self::type_of(expr, env)?;
// 				let casting_error = Err(ZephyrError::UnassignableTypes(
// 					expr_type.clone(),
// 					to_type.clone(),
// 				));
//
// 				if &expr_type == to_type {
// 					return self.compile_expr(expr, env);
// 				}
//
// 				match (expr_type, to_type) {
// 					(TypeDef::String, TypeDef::List(list_type)) => {
// 						if matches!(list_type.as_ref(), TypeDef::Char) {
// 							Ok(format!("{}.split('')", self.compile_expr(expr, env)?))
// 						} else {
// 							casting_error
// 						}
// 					}
// 					(TypeDef::List(list_type), TypeDef::String) => {
// 						if matches!(list_type.as_ref(), TypeDef::Char) {
// 							Ok(format!("{}.join('')", self.compile_expr(expr, env)?))
// 						} else {
// 							casting_error
// 						}
// 					}
// 					(TypeDef::Int, TypeDef::Float) => self.compile_expr(expr, env),
// 					(TypeDef::Float, TypeDef::Int) => {
// 						Ok(format!("Math.floor({})", self.compile_expr(expr, env)?))
// 					}
// 					(TypeDef::Union(orig), TypeDef::Union(new)) => {
// 						if orig.is_subset(new) {
// 							self.compile_expr(expr, env)
// 						} else {
// 							casting_error
// 						}
// 					}
// 					(orig, TypeDef::Union(new)) => {
// 						if new.contains(&orig) {
// 							self.compile_expr(expr, env)
// 						} else {
// 							casting_error
// 						}
// 					}
// 					(TypeDef::Map(key, value), TypeDef::Object(fields)) => {
// 						if key.assignable_to(&TypeDef::String)
// 							&& fields.values().all(|t| value.assignable_to(t))
// 						{
// 							Ok(format!(
// 								"Object.fromEntries(({}).entries())",
// 								self.compile_expr(expr, env)?
// 							))
// 						} else {
// 							casting_error
// 						}
// 					}
// 					(TypeDef::Object(fields), TypeDef::Map(key, value)) => {
// 						if key.assignable_to(&TypeDef::String)
// 							&& fields.values().all(|t| t.assignable_to(value))
// 						{
// 							Ok(format!(
// 								"new Map(Object.entries({}))",
// 								self.compile_expr(expr, env)?
// 							))
// 						} else {
// 							casting_error
// 						}
// 					}
// 					(_, _) => casting_error,
// 				}
// 			}
// 			Expr::TypeCheck {
// 				expr: _,
// 				is_type: _,
// 			} => {
// 				todo!()
// 			}
// 			Expr::CodeBlock(body) => {
// 				let body = body
// 					.iter()
// 					.map(|e| self.compile_expr(e, env))
// 					.try_collect::<String, String, _>()
// 					.iter()
// 					.join(";");
//
// 				Ok("{".to_string() + &body + "}")
// 			}
// 		}
// 	}
//
// 	fn type_of<'a>(
// 		expr: &'a Expr<'a>,
// 		env: &CompilerEnv<'a>,
// 	) -> Result<TypeDef<'a>, ZephyrError<'a>>
// where {
// 		match expr {
// 			Expr::Int(_) => Ok(TypeDef::Int),
// 			Expr::Float(_) => Ok(TypeDef::Float),
// 			Expr::Char(_) => Ok(TypeDef::Char),
// 			Expr::String(_) => Ok(TypeDef::String),
// 			Expr::Boolean(_) => Ok(TypeDef::Boolean),
// 			Expr::List(list) => Ok(TypeDef::List(if let Some(first) = list.first() {
// 				Self::type_of(first, env)?.into()
// 			} else {
// 				TypeDef::Void.into()
// 			})),
// 			Expr::Map(map) => Ok(if let Some((k, v)) = map.first_key_value() {
// 				TypeDef::Map(Self::type_of(k, env)?.into(), Self::type_of(v, env)?.into())
// 			} else {
// 				TypeDef::Map(TypeDef::Void.into(), TypeDef::Void.into())
// 			}),
// 			Expr::Tuple(tuple) => Ok(TypeDef::Tuple(
// 				tuple.iter().map(|t| Self::type_of(t, env)).try_collect()?,
// 			)),
// 			Expr::Object(obj) => Ok(TypeDef::Object(
// 				obj
// 					.iter()
// 					.map(
// 						|(
// 							k,
// 							ObjectData {
// 								value, type_def, ..
// 							},
// 						)| {
// 							Ok((
// 								k,
// 								Box::new(if let Some(type_def) = type_def {
// 									type_def.clone()
// 								} else {
// 									Self::type_of(value, env)?
// 								}),
// 							))
// 						},
// 					)
// 					.map_ok(|(&k, v)| (k, v))
// 					.try_collect()?,
// 			)),
// 			Expr::Closure {
// 				params,
// 				return_type,
// 				body,
// 			} => Ok(TypeDef::Closure {
// 				params: params.values().cloned().collect(),
// 				return_type: if let Some(return_type) = return_type {
// 					return_type.clone()
// 				} else {
// 					Self::type_of(body, env)?
// 				}
// 				.into(),
// 			}),
// 			Expr::Ident(name) | Expr::Assignment { ident: name, .. } => match env.get(name) {
// 				Some(ident) => Ok(ident.type_def.clone()),
// 				None => Err(ZephyrError::UnknownIdent(name)),
// 			},
// 			Expr::Call { callee, .. } => {
// 				let closure_type = Self::type_of(callee, env)?;
// 				if let TypeDef::Closure { return_type, .. } = closure_type {
// 					Ok(*return_type)
// 				} else {
// 					// calling non-closures will be caught before this gets called
// 					Err(ZephyrError::NotAClosure(closure_type))
// 				}
// 			}
// 			Expr::MemberAccess { parent, member } => {
// 				let parent_type = Self::type_of(parent, env)?;
// 				if let TypeDef::Object(fields) = parent_type {
// 					fields
// 						.get(member)
// 						.map(|t| *t.clone())
// 						.ok_or(ZephyrError::NoSuchField(
// 							member,
// 							fields.keys().cloned().collect_vec(),
// 						))
// 				} else {
// 					Err(ZephyrError::MemberAccessOnNonObject(parent_type))
// 				}
// 			}
// 			Expr::IndexAccess { parent, index } => {
// 				let parent_type = Self::type_of(parent, env)?;
// 				let expected_index_type = parent_type
// 					.index_type()
// 					.ok_or(ZephyrError::UnindexableValue(parent_type.clone()))?;
// 				let index_type = Self::type_of(index, env)?;
//
// 				if index_type.assignable_to(expected_index_type) {
// 					Ok(TypeDef::Union(BTreeSet::from([
// 						TypeDef::Void,
// 						match parent_type {
// 							TypeDef::String => TypeDef::Char,
// 							TypeDef::List(item) => *item,
// 							TypeDef::Map(_, value) => *value,
// 							TypeDef::Tuple(items) => {
// 								if let Expr::Int(i) = index.as_ref() {
// 									if *i < 0 {
// 										Err(ZephyrError::NegativeIndexOnTuples)?
// 									} else {
// 										items
// 											.get(*i as usize)
// 											.ok_or(ZephyrError::IndexOutOfBounds(*i, items.len()))?
// 											.clone()
// 									}
// 								} else {
// 									TypeDef::Union(BTreeSet::from_iter(items.iter().cloned()))
// 								}
// 							}
// 							TypeDef::Object(fields) => {
// 								if let Expr::String(key) = index.as_ref() {
// 									*fields
// 										.get(key.as_str())
// 										.ok_or(ZephyrError::NoSuchField(
// 											key,
// 											fields.keys().cloned().collect(),
// 										))?
// 										.clone()
// 								} else {
// 									TypeDef::Union(BTreeSet::from_iter(
// 										fields.values().map(|t| t.as_ref()).cloned(),
// 									))
// 								}
// 							}
// 							_ => unreachable!(),
// 						},
// 					])))
// 				} else {
// 					let expected_index = expected_index_type.clone();
// 					Err(ZephyrError::WrongIndex(
// 						parent_type,
// 						index_type,
// 						expected_index,
// 					))
// 				}
// 			}
// 			Expr::BinaryOp { op, lhs, .. } => match op {
// 				BinaryOp::Plus
// 				| BinaryOp::Minus
// 				| BinaryOp::Times
// 				| BinaryOp::Divide
// 				| BinaryOp::Modulus
// 				| BinaryOp::Power => Self::type_of(lhs, env),
// 				BinaryOp::Equals
// 				| BinaryOp::NotEquals
// 				| BinaryOp::LessThan
// 				| BinaryOp::LessThanEquals
// 				| BinaryOp::GreaterThan
// 				| BinaryOp::GreaterThanEquals
// 				| BinaryOp::And
// 				| BinaryOp::Or
// 				| BinaryOp::In
// 				| BinaryOp::NotIn => Ok(TypeDef::Boolean),
// 			},
// 			Expr::PrefixOp { op, expr } => match op {
// 				PrefixOp::Not => Ok(TypeDef::Boolean),
// 				PrefixOp::Negate => Self::type_of(expr, env), // this is either a number or a float, any other type will be caught earlier when parsing the expr
// 			},
// 			Expr::PostfixOp { expr, .. } => Self::type_of(expr, env),
// 			Expr::Let(
// 				_,
// 				VarData {
// 					type_def, value, ..
// 				},
// 			) => {
// 				if let Some(type_def) = type_def {
// 					Ok(type_def.clone())
// 				} else {
// 					Self::type_of(value, env)
// 				}
// 			}
// 			Expr::If {
// 				then, otherwise, ..
// 			} => otherwise.as_ref().map_or(Ok(TypeDef::Void), |t| {
// 				Ok(TypeDef::Union(BTreeSet::from([
// 					Self::type_of(then, env)?,
// 					Self::type_of(t, env)?,
// 				])))
// 			}),
// 			Expr::While { .. } | Expr::For { .. } => Ok(TypeDef::Void),
// 			Expr::Import { .. } => todo!(),
// 			Expr::Cast { to_type, .. } => Ok(to_type.clone()),
// 			Expr::TypeCheck { .. } => Ok(TypeDef::Boolean),
// 			Expr::CodeBlock(body) => body
// 				.last()
// 				.map_or(Ok(TypeDef::Void), |t| Self::type_of(t, env)),
// 		}
// 	}
// }
//
// impl Default for Compiler<'_> {
// 	fn default() -> Self {
// 		Self {
// 			top_level: Cow::Owned(BTreeMap::default()),
// 		}
// 	}
// }
