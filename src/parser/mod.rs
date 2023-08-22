use std::{
	collections::BTreeMap,
	io::{Error, ErrorKind},
};

use color_eyre::eyre::Result;
use itertools::Itertools;
use pest::{iterators::Pair, iterators::Pairs, pratt_parser::PrattParser};
use pest_derive::Parser;
use unescape::unescape;

use crate::parser::{
	ast::{AssignOp, BinaryOp, ObjectData, PostfixOp, PrefixOp, ZephyrExpression, ZephyrType},
	error::ZephyrError,
};

mod ast;
mod error;

#[derive(Parser)]
#[grammar = "parser/grammar.pest"]
pub struct ZephyrParser;

lazy_static::lazy_static! {
	static ref PRATT_PARSER: PrattParser<Rule> = {
		use pest::pratt_parser::{Assoc::*, Op};
		use Rule::*;

		PrattParser::new()
			// containment
			.op(Op::infix(in_op, Left) | Op::infix(not_in, Left))
			// comparison/equality/boolean
			.op(
				Op::infix(equals, Left)
					| Op::infix(not_equals, Left)
					| Op::infix(less_than, Left)
					| Op::infix(less_equals, Left)
					| Op::infix(greater_than, Left)
					| Op::infix(greater_equals, Left)
					| Op::infix(and, Left)
					| Op::infix(or, Left),
			)
			// prefix ops
			.op(Op::prefix(not) | Op::prefix(negate))
			// addition/subtraction
			.op(Op::infix(plus, Left) | Op::infix(minus, Left))
			// multiplication/division/modulus
			.op(Op::infix(times, Left) | Op::infix(divide, Left) | Op::infix(modulus, Left))
			// exponentiation
			.op(Op::infix(power, Right))
			// incrementation/decrementation
			.op(Op::postfix(increment) | Op::postfix(decrement))
	};
}

type ParseResult<'a> = Result<ZephyrExpression<'a>, ZephyrError<'a>>;
impl ZephyrParser {
	pub fn parse_expr(pairs: Pairs<Rule>) -> ParseResult {
		PRATT_PARSER
			.map_primary(Self::parse_primary)
			.map_prefix(Self::map_prefix)
			.map_postfix(Self::map_postfix)
			.map_infix(Self::map_infix)
			.parse(pairs)
	}

	fn parse_primary(primary: Pair<Rule>) -> ParseResult {
		match primary.as_rule() {
			Rule::float => Self::parse_float(primary),
			Rule::integer => Self::parse_int(primary),
			Rule::char => Self::parse_char(primary),
			Rule::string => Self::parse_string(primary),
			Rule::boolean => Self::parse_boolean(primary),
			Rule::list => Self::parse_list(primary),
			Rule::map => Self::parse_map(primary),
			Rule::tuple => Self::parse_tuple(primary),
			Rule::object => Self::parse_object(primary),
			Rule::closure => Self::parse_closure(primary),
			Rule::call => Self::parse_call(primary),
			Rule::let_expr => Self::parse_let(primary),
			Rule::identifier => Self::parse_identifier(primary),
			Rule::assignment => Self::parse_assignment(primary),
			Rule::if_expr => Self::parse_if(primary),
			Rule::while_expr => Self::parse_while(primary),
			Rule::for_expr => Self::parse_for(primary),
			Rule::import => Self::parse_import(primary),
			Rule::code_block => Self::parse_code_block(primary),
			Rule::cast | Rule::type_check => Self::parse_type_op(primary),
			Rule::parens => Ok(Self::parse_expr(
				primary.into_inner().next().unwrap().into_inner(),
			)?),
			Rule::primary => Self::parse_primary(primary.into_inner().next().unwrap()),
			// this shouldn't happen, but should still work to recover
			Rule::expr => Self::parse_expr(primary.into_inner()),
			_ => {
				println!("{:?}", primary.as_rule());
				unreachable!()
			}
		}
	}

	fn map_infix<'a>(lhs: ParseResult<'a>, op: Pair<Rule>, rhs: ParseResult<'a>) -> ParseResult<'a> {
		Ok(ZephyrExpression::BinaryOp {
			op: match op.as_rule() {
				Rule::in_op => BinaryOp::In,
				Rule::not_in => BinaryOp::NotIn,
				Rule::not_equals => BinaryOp::NotEquals,
				Rule::equals => BinaryOp::Equals,
				Rule::plus => BinaryOp::Plus,
				Rule::minus => BinaryOp::Minus,
				Rule::times => BinaryOp::Times,
				Rule::divide => BinaryOp::Divide,
				Rule::modulus => BinaryOp::Modulus,
				Rule::power => BinaryOp::Power,
				Rule::less_equals => BinaryOp::LessThanEquals,
				Rule::less_than => BinaryOp::LessThan,
				Rule::greater_equals => BinaryOp::GreaterThanEquals,
				Rule::greater_than => BinaryOp::GreaterThan,
				Rule::and => BinaryOp::And,
				Rule::or => BinaryOp::Or,
				_ => unreachable!(),
			},
			lhs: lhs?.into(),
			rhs: rhs?.into(),
		})
	}

	fn map_postfix<'a>(expr: ParseResult<'a>, op: Pair<Rule>) -> ParseResult<'a> {
		Ok(ZephyrExpression::PostfixOp {
			op: match op.as_rule() {
				Rule::increment => PostfixOp::Increment,
				Rule::decrement => PostfixOp::Decrement,
				_ => unreachable!(),
			},
			expr: expr?.into(),
		})
	}

	fn map_prefix<'a>(op: Pair<Rule>, expr: ParseResult<'a>) -> ParseResult<'a> {
		Ok(ZephyrExpression::PrefixOp {
			op: match op.as_rule() {
				Rule::not => PrefixOp::Not,
				Rule::negate => PrefixOp::Negate,
				_ => unreachable!(),
			},
			expr: expr?.into(),
		})
	}

	fn parse_float<'a>(primary: Pair<Rule>) -> ParseResult<'a> {
		Ok(ZephyrExpression::Float(
			primary
				.as_str()
				.parse::<f64>()
				.map_err(|e| ZephyrError::ParseError(e.into()))?
				.into(),
		))
	}

	fn parse_int<'a>(primary: Pair<Rule>) -> ParseResult<'a> {
		Ok(ZephyrExpression::Int(
			primary
				.as_str()
				.parse::<i64>()
				.map_err(|e| ZephyrError::ParseError(e.into()))?,
		))
	}

	fn parse_char<'a>(primary: Pair<Rule>) -> ParseResult<'a> {
		Ok(ZephyrExpression::Char(
			unescape(primary.into_inner().next().unwrap().as_str())
				.ok_or(ZephyrError::ParseError(
					Error::from(ErrorKind::InvalidData).into(),
				))?
				.parse::<char>()
				.map_err(|e| ZephyrError::ParseError(e.into()))?,
		))
	}

	fn parse_string<'a>(primary: Pair<Rule>) -> ParseResult<'a> {
		Ok(ZephyrExpression::String(
			unescape(primary.into_inner().next().unwrap().as_str()).ok_or(ZephyrError::ParseError(
				Error::from(ErrorKind::InvalidData).into(),
			))?,
		))
	}

	fn parse_boolean<'a>(primary: Pair<Rule>) -> ParseResult<'a> {
		Ok(ZephyrExpression::Boolean(
			primary
				.as_str()
				.parse::<bool>()
				.map_err(|e| ZephyrError::ParseError(e.into()))?,
		))
	}

	fn parse_list(primary: Pair<Rule>) -> ParseResult {
		Ok(ZephyrExpression::List(
			primary
				.into_inner()
				.map(|pair| Self::parse_expr(pair.into_inner()))
				.try_collect()?,
		))
	}

	fn parse_map(primary: Pair<Rule>) -> ParseResult {
		Ok(ZephyrExpression::Map(
			primary
				.into_inner()
				.map(|pair| {
					let mut inner = pair.into_inner();
					let key_pair = inner
						.next()
						.ok_or(ZephyrError::ParseError(
							Error::from(ErrorKind::InvalidData).into(),
						))?
						.into_inner();
					let value_pair = inner
						.next()
						.ok_or(ZephyrError::ParseError(
							Error::from(ErrorKind::InvalidData).into(),
						))?
						.into_inner();
					let key_expr = Self::parse_expr(key_pair)?;
					let value_expr = Self::parse_expr(value_pair)?;

					Ok((key_expr, value_expr))
				})
				.try_collect()?,
		))
	}

	fn parse_tuple(primary: Pair<Rule>) -> ParseResult {
		Ok(ZephyrExpression::Tuple(
			primary
				.into_inner()
				.map(|pair| Self::parse_expr(pair.into_inner()))
				.try_collect()?,
		))
	}

	fn parse_object(primary: Pair<Rule>) -> ParseResult {
		Ok(ZephyrExpression::Object(
			primary
				.into_inner()
				.map(|pair| {
					let mut primary = pair.into_inner();
					let mutable = matches!(primary.next().unwrap().as_rule(), Rule::mutable);
					let field_name = primary.next().unwrap().as_str();

					let type_def_or_value = primary.next().unwrap();
					Ok(match type_def_or_value.as_rule() {
						Rule::type_def => {
							let type_def = Self::parse_type_def(type_def_or_value)?.into();
							let value_pair = primary.next().ok_or(ZephyrError::ParseError(
								Error::from(ErrorKind::InvalidData).into(),
							))?;
							let value = Self::parse_expr(value_pair.into_inner())?;

							(
								field_name,
								ObjectData {
									value,
									mutable,
									type_def,
								},
							)
						}
						Rule::expr => {
							let value = Self::parse_expr(type_def_or_value.into_inner())?;

							(
								field_name,
								ObjectData {
									value,
									mutable,
									type_def: None,
								},
							)
						}
						_ => unreachable!(),
					})
				})
				.try_collect()?,
		))
	}

	fn parse_closure(primary: Pair<Rule>) -> ParseResult {
		let mut primary = primary.into_inner();
		let params = primary
			.take_while_inclusive(|pair| matches!(pair.as_rule(), Rule::closure_param))
			.map(|param_pair| {
				let mut param = param_pair.into_inner();
				let name = param.next().unwrap().as_str();
				let type_def = Self::parse_type_def(param.next().unwrap())?;

				Ok((name, type_def))
			})
			.try_collect()?;

		let ret_type_or_body = primary.next().unwrap();

		if matches!(ret_type_or_body.as_rule(), Rule::type_def) {
			// it was a return type
			let body = Self::parse_expr(primary.next().unwrap().into_inner())?.into();

			Ok(ZephyrExpression::Closure {
				params,
				return_type: Some(Self::parse_type_def(ret_type_or_body)?),
				body,
			})
		} else {
			Ok(ZephyrExpression::Closure {
				params,
				return_type: None,
				body: Self::parse_expr(ret_type_or_body.into_inner())?.into(),
			})
		}
	}

	fn parse_call(primary: Pair<Rule>) -> ParseResult {
		let mut primary = primary.into_inner();
		let callee_pair = primary
			.next()
			.unwrap() // goes into call_term
			.into_inner()
			.next()
			.unwrap(); // this is either an ident or an expr with parens
		let callee = match callee_pair.as_rule() {
			Rule::identifier => ZephyrExpression::Ident(callee_pair.as_str()),
			Rule::expr => Self::parse_expr(callee_pair.clone().into_inner())?,
			_ => unreachable!(),
		}
		.into();

		Ok(ZephyrExpression::Call {
			callee,
			args: primary
				.map(|pair| Self::parse_expr(pair.into_inner()))
				.try_collect()?,
		})
	}

	fn parse_let(primary: Pair<Rule>) -> ParseResult {
		let mut primary = primary.into_inner();

		let mutable = matches!(primary.next().unwrap().as_rule(), Rule::mutable);
		let name = primary.next().unwrap().as_str();

		let type_def_or_value = primary.next().unwrap();
		Ok(match type_def_or_value.as_rule() {
			Rule::type_def => {
				let type_def = Self::parse_type_def(type_def_or_value)?.into();
				let value_pair = primary.next().ok_or(ZephyrError::ParseError(
					Error::from(ErrorKind::InvalidData).into(),
				))?;
				let value = Self::parse_expr(value_pair.into_inner())?.into();

				ZephyrExpression::Let {
					name,
					mutable,
					type_def,
					value,
				}
			}
			Rule::expr => {
				let value = Self::parse_expr(type_def_or_value.into_inner())?.into();

				ZephyrExpression::Let {
					name,
					value,
					mutable,
					type_def: None,
				}
			}
			_ => unreachable!(),
		})
	}

	fn parse_identifier(primary: Pair<Rule>) -> ParseResult {
		Ok(ZephyrExpression::Ident(primary.as_str()))
	}

	fn parse_assignment(primary: Pair<Rule>) -> ParseResult {
		let mut primary = primary.into_inner();
		let ident = primary.next().unwrap().as_str();
		let op = match primary.next().unwrap().as_rule() {
			Rule::assign => AssignOp::None,
			Rule::plus_assign => AssignOp::Plus,
			Rule::minus_assign => AssignOp::Minus,
			Rule::times_assign => AssignOp::Times,
			Rule::divide_assign => AssignOp::Divide,
			Rule::modulus_assign => AssignOp::Modulus,
			Rule::power_assign => AssignOp::Power,
			Rule::and_assign => AssignOp::And,
			Rule::or_assign => AssignOp::Or,
			_ => unreachable!(),
		};
		let new = Self::parse_expr(primary.next().unwrap().into_inner())?.into();

		Ok(ZephyrExpression::Assignment { ident, op, new })
	}

	fn parse_if(primary: Pair<Rule>) -> ParseResult {
		let mut primary = primary.into_inner();
		let condition = Self::parse_expr(primary.next().unwrap().into_inner())?.into();
		let then = Self::parse_expr(primary.next().unwrap().into_inner())?.into();
		let otherwise = primary
			.next()
			.map(|pair| Self::parse_expr(pair.into_inner()));

		Ok(ZephyrExpression::If {
			condition,
			then,
			otherwise: match otherwise {
				Some(else_) => Some(else_?.into()),
				None => None,
			},
		})
	}

	fn parse_while(primary: Pair<Rule>) -> ParseResult {
		let mut primary = primary.into_inner();
		let condition = Self::parse_expr(primary.next().unwrap().into_inner())?.into();
		let body = Self::parse_expr(primary.next().unwrap().into_inner())?.into();

		Ok(ZephyrExpression::While { condition, body })
	}

	fn parse_for(primary: Pair<Rule>) -> ParseResult {
		let mut primary = primary.into_inner();
		let name = primary.next().unwrap().as_str();
		let iterable = Self::parse_expr(primary.next().unwrap().into_inner())?.into();
		let body = Self::parse_expr(primary.next().unwrap().into_inner())?.into();

		Ok(ZephyrExpression::For {
			name,
			iterable,
			body,
		})
	}

	fn parse_import(primary: Pair<Rule>) -> ParseResult {
		let mut primary = primary.into_inner();
		let path = unescape(primary.next().unwrap().as_str())
			.ok_or(ZephyrError::ParseError(
				Error::from(ErrorKind::InvalidData).into(),
			))?
			.clone();
		// if there is no idents clause, the entire module is imported by default
		// if the clause is empty, it is the same as importing an empty module
		let idents = primary.next().map(|idents_clause| {
			idents_clause
				.into_inner()
				.map(|pair| {
					let mut pair = pair.into_inner();
					let ident = pair.next().unwrap().as_str();
					let alias = pair.next().map(|x| x.as_str());

					(ident, alias)
				})
				.collect::<BTreeMap<_, _>>()
		});

		Ok(ZephyrExpression::Import { path, idents })
	}

	fn parse_type_op(primary: Pair<Rule>) -> ParseResult {
		let mut pairs = primary.clone().into_inner();
		let expr = Self::parse_expr(pairs.next().unwrap().into_inner())?.into();
		let type_arg = Self::parse_type_def(pairs.next().unwrap())?;

		match primary.as_rule() {
			Rule::cast => Ok(ZephyrExpression::Cast {
				expr,
				to_type: type_arg,
			}),
			Rule::type_check => Ok(ZephyrExpression::TypeCheck {
				expr,
				is_type: type_arg,
			}),
			_ => unreachable!(),
		}
	}

	fn parse_code_block(primary: Pair<Rule>) -> ParseResult {
		Ok(ZephyrExpression::CodeBlock(
			primary
				.into_inner()
				.map(|pair| Self::parse_expr(pair.into_inner()))
				.try_collect()?,
		))
	}

	fn parse_type_def(pair: Pair<Rule>) -> Result<ZephyrType, ZephyrError> {
		match pair.as_rule() {
			Rule::type_def => {
				let union_elems = pair.into_inner();
				if union_elems.len() == 1 {
					let type_def = &union_elems.collect_vec()[0];
					Self::parse_type_def(type_def.clone())
				} else {
					Ok(ZephyrType::Union(
						union_elems.map(Self::parse_type_def).try_collect()?,
					))
				}
			}
			Rule::int_type => Ok(ZephyrType::Int),
			Rule::float_type => Ok(ZephyrType::Float),
			Rule::char_type => Ok(ZephyrType::Char),
			Rule::string_type => Ok(ZephyrType::String),
			Rule::boolean_type => Ok(ZephyrType::Boolean),
			Rule::void_type => Ok(ZephyrType::Void),
			Rule::list_type => Ok(ZephyrType::List(
				Self::parse_type_def(pair.into_inner().next().unwrap())?.into(),
			)),
			Rule::map_type => Ok({
				let key_pair = pair.clone().into_inner().next().unwrap();
				let value_pair = pair.into_inner().next().unwrap();
				ZephyrType::Map(
					Self::parse_type_def(key_pair)?.into(),
					Self::parse_type_def(value_pair)?.into(),
				)
			}),
			Rule::tuple_type => Ok(ZephyrType::Tuple(
				pair.into_inner().map(Self::parse_type_def).try_collect()?,
			)),
			Rule::object_type => Ok(ZephyrType::Object(
				pair
					.into_inner()
					.map(|pair| {
						let mut inner = pair.into_inner();
						let field_name = inner.next().unwrap().as_str();
						let value_pair = inner.next().unwrap();
						let field_value = Self::parse_type_def(value_pair)?;

						Ok((field_name, field_value.into()))
					})
					.try_collect()?,
			)),
			Rule::closure_type => Ok({
				let params_and_ret = pair
					.into_inner()
					.map(Self::parse_type_def)
					.try_collect::<_, Vec<_>, _>()?;

				let (return_type, params) = params_and_ret.split_last().unwrap();

				ZephyrType::Closure {
					params: params.to_vec(),
					return_type: return_type.clone().into(),
				}
			}),
			Rule::type_primary => Self::parse_type_def(pair.into_inner().next().unwrap()),
			_ => {
				println!("{:?}", pair.as_rule());
				unreachable!()
			}
		}
	}
}
