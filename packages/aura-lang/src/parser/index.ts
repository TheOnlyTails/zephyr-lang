import { z } from "zod"
import {
	assignmentOperations,
	type AuraExpression,
	type AuraProgram,
	type AuraType,
	type BinaryOp,
	binaryOpResult,
	booleanOperations,
	comparisonOperations,
	equalityOperations,
	inclusionOperations,
	numberOperations,
	type UnaryOp,
	unaryOpResult
} from "./ast"
import grammar, { type AuraActionDict } from "./grammar.ohm-bundle.js"
import { assignableTo, suggestions, typeName } from "./utils"
import * as crypto from "node:crypto"
import { match } from "ts-pattern"
import { deepEqual } from "fast-equals"
import { type AuraOptions, defaultOptions } from ".."
import { findUpSync } from "find-up"
import path from "node:path"

type Actions<Start extends string, ReturnType> = Required<{
	[K in keyof AuraActionDict<ReturnType> as K extends `${Start}${string}`
		? K
		: never]: AuraActionDict<ReturnType>[K]
}>

export const parse = <Ret extends "type" | "expr" | undefined = undefined>(
	source: string,
	result?: Ret,
	opts: AuraOptions = {}
) => {
	const { src, config } = { ...defaultOptions, ...opts }
	const root = findUpSync(config)
	if (!root) throw new Error(`Could not find the project root with a config file named "${config}".`)
	const basePath = path.resolve(root, src)

	const env = {
		bindings: new Map<
			string,
			{
				type: AuraType
				value: AuraExpression
				mutable: boolean
				scope?: string[]
			}
		>(),
		types: new Map<string, { value: AuraType; scope: string[] }>([
			["int", { value: { type: "int" }, scope: [] }],
			["float", { value: { type: "float" }, scope: [] }],
			["char", { value: { type: "char" }, scope: [] }],
			["string", { value: { type: "string" }, scope: [] }],
			["boolean", { value: { type: "boolean" }, scope: [] }],
			["void", { value: { type: "void" }, scope: [] }]
		])
	}

	// evaluates a type reference
	const semantics = grammar
		.createSemantics()
		.addOperation<AuraType>("type(scope)", {
			Type_int: (_) => ({ type: "int" }),
			Type_float: (_) => ({ type: "float" }),
			Type_char: (_) => ({ type: "char" }),
			Type_string: (_) => ({ type: "string" }),
			Type_boolean: (_) => ({ type: "boolean" }),
			Type_void: (_) => ({ type: "void" }),
			Type_closure(_, params, _1, _2, _3, returnType) {
				return {
					type: "closure",
					params: params.asIteration().children.map((param) => param.type(this.args.scope)),
					returnType: returnType.type(this.args.scope)
				}
			},
			Type_list(_, elementType, _1) {
				return {
					type: "list",
					elementType: elementType.type(this.args.scope)
				}
			},
			Type_map(_, keyType, _2, valueType, _3) {
				return {
					type: "map",
					keyType: keyType.type(this.args.scope) as AuraType,
					valueType: valueType.type(this.args.scope) as AuraType
				}
			},
			Type_tuple(_, items, _1, _2) {
				return {
					type: "tuple",
					items: items.asIteration().children.map(t => t.type(this.args.scope) as AuraType)
				}
			},
			Type_object(_, fields, _1, _2) {
				return {
					type: "object",
					fields: new Map(fields.asIteration().children.map(entry => [
						entry.child(0).sourceString,
						entry.child(2).type(this.args.scope) as AuraType
					]))
				}
			},
			Type_union(types) {
				return {
					type: "union",
					types: types.asIteration().children.map((type) => type.type(this.args.scope) as AuraType)
						.flatMap((type) => type.type === "union" ? type.types : [type])
				}
			},
			Type_ident(name, _, generics, _1, _2) {
				if (
					env.types
						.get(name.sourceString)
						?.scope.some((layer: string) => !(this.args.scope as string[]).includes(layer))
				)
					throw Error(
						`Could not find type ${name.sourceString} because it ${
							env.types.has(name.sourceString)
								? "is not in scope"
								: suggestions("does not exist", name.sourceString, env.types.keys())
						}.`
					)

				return {
					type: "identifier",
					name: name.sourceString,
					generics: generics.children.map((generic) => generic.sourceString)
				}
			},
			Type_parens(_, inner, _1) {
				return inner.type(this.args.scope)
			}
		} satisfies Actions<"Type_", AuraType>)

		// returns the type of an expression without evaluating it
		.addOperation<AuraType>("typeOf(scope)", {
			Expression_int: (_) => ({ type: "int" }),
			Expression_float: (_) => ({ type: "float" }),
			Expression_char: (_) => ({ type: "char" }),
			Expression_string: (_) => ({ type: "string" }),
			Expression_boolean: (_) => ({ type: "boolean" }),
			Expression_closure(_, params, _1, _2, _3, body) {
				return {
					type: "closure",
					params: params.children.map((param) => param.type(this.args.scope)),
					returnType: body.typeOf(this.args.scope)
				}
			},
			Expression_list(_, elements, _1, _2) {
				return {
					type: "list",
					elementType: (elements.asIteration().child(0)?.typeOf(this.args.scope) as AuraType) ?? {
						type: "void"
					}
				}
			},
			Expression_map(_, entries, _1, _2) {
				const firstEntry = entries.asIteration().child(0)

				return {
					type: "map",
					keyType: firstEntry?.child(0)?.typeOf(this.args.scope) ?? { type: "void" } as AuraType,
					valueType: firstEntry?.child(2)?.typeOf(this.args.scope) ?? { type: "void" } as AuraType
				} as AuraType
			},
			Expression_tuple(_, items, _1, _2) {
				return {
					type: "tuple",
					items: items.asIteration().children.map(n => n.typeOf(this.args.scope) as AuraType)
				}
			},
			Expression_object(_, fields, _2, _3) {
				return {
					type: "object",
					fields: new Map(
						fields.asIteration().children.map((node) => [
							node.child(0).sourceString,
							node.child(2).typeOf(this.args.scope) as AuraType
						])
					)
				}
			},
			Expression_ident(name) {
				const binding = env.bindings.get(name.sourceString)

				if (binding) return binding.type
				else {
					throw new Error(
						suggestions(
							`Unknown identifier ${name.sourceString}.`,
							name.sourceString,
							env.bindings.keys()
						)
					)
				}
			},
			Expression_closure_call(callee, _, _1, _2, _3) {
				const calleeType = callee.typeOf(this.args.scope) as AuraType
				if (calleeType.type === "closure") return calleeType.returnType
				else throw new Error(`Cannot call a value of type ${calleeType.type}`)
			},
			Expression_parens(_, expr, _1) {
				return expr.typeOf(this.args.scope)
			},
			Expression_binary_op(lhs, op, rhs) {
				const lhsType = lhs.typeOf(this.args.scope) as AuraType
				const rhsType = rhs.typeOf(this.args.scope) as AuraType

				return binaryOpResult[op.sourceString as BinaryOp](lhsType, rhsType)
			},
			Expression_code_block(_, body, _1) {
				return (
					body.asIteration().children.at(-1)?.typeOf(this.args.scope) ?? {
						type: "void"
					}
				)
			},
			Expression_for(_, _1, _2, _3, _4, _5, _6) {
				return { type: "void" }
			},
			Expression_if(_, _1, _2, _3, then, _4, otherwise) {
				if (otherwise.sourceString === "") return { type: "void" }

				const thenType = then.typeOf(this.args.scope) as AuraType
				const otherwiseType = otherwise.typeOf(this.args.scope) as AuraType
				if (thenType !== otherwiseType)
					throw new Error(
						`Both branches of an if-else expression must have the same type, found ${typeName(
							thenType
						)} and ${typeName(otherwiseType)}.`
					)

				return thenType
			},
			Expression_let(_, _1, _2, _3, typeAnn, _4, value) {
				if (typeAnn.sourceString !== "") return typeAnn.type(this.args.scope) as AuraType
				else return value.typeOf(this.args.scope) as AuraType
			},
			Expression_unary_op_postfix(operand, op) {
				return unaryOpResult[op.sourceString as UnaryOp](
					operand.typeOf(this.args.scope) as AuraType
				)
			},
			Expression_unary_op_prefix(op, operand) {
				return unaryOpResult[op.sourceString as UnaryOp](
					operand.typeOf(this.args.scope) as AuraType
				)
			},
			Expression_while(_, _1, _2, _3, _4) {
				return { type: "void" }
			},
			Expression_import(_, path, _2, idents, _4, _5) {
				return {
					type: "object",
					fields: new Map(idents.asIteration().children.map())
				}
			},
			Expression_cast(expr, _, type) {
				return type.type(this.args.scope) as AuraType
			},
			Expression_type_check(expr, _, targetType) {
				return { type: "boolean" }
			},
		} satisfies Actions<"Expression_", AuraType>)
		.addOperation<AuraExpression>("expr(scope)", {
			Expression_int: (val) => ({
				type: "int",
				value: z.coerce.number().int().parse(val.sourceString.replaceAll("_", ""))
			}),
			Expression_float: (val) => ({
				type: "float",
				value: z.coerce.number().parse(val.sourceString.replaceAll("_", ""))
			}),
			Expression_char: (val) => {
				let value = val.sourceString.slice(1, -1)

				value = value
					.replace("\\n", "\n")
					.replace("\\t", "\t")
					.replace("\\r", "\r")
					.replace("\\\"", "\"")
					.replace("\\'", "'")
					.replace("\\\\", "\\")

				for (const codePoint of value.matchAll(/\\u([\da-fA-F]{4})/g))
					value = value.replace(
						codePoint[0],
						String.fromCharCode(parseInt(codePoint[1], 16)).normalize()
					)

				return {
					type: "char",
					value: z.string().parse(value)
				}
			},
			Expression_string: (val) => {
				let value = val.sourceString.slice(1, -1)
				value = value
					.replace("\\n", "\n")
					.replace("\\t", "\t")
					.replace("\\r", "\r")
					.replace("\\\"", "\"")
					.replace("\\'", "'")
					.replace("\\\\", "\\")

				for (const codePoint of value.matchAll(/\\u([\da-fA-F]{4})/g))
					value = value.replace(
						codePoint[0],
						String.fromCharCode(parseInt(codePoint[1], 16)).normalize()
					)

				return {
					type: "string",
					value
				}
			},
			Expression_boolean: (val) => ({
				type: "boolean",
				value: match(val.sourceString)
					.with("true", () => true)
					.with("false", () => false)
					.otherwise((val) => {
						throw new Error(`A boolean literal must be either "true" or "false", not ${val}`)
					})
			}),
			Expression_closure(_, params, _1, _2, _3, body) {
				const closureScopeId = crypto.randomUUID()
				const returnType = body.typeOf([...this.args.scope, closureScopeId]) as AuraType
				const paramTypes = params.asIteration().children.map(n => n.child(2).type(this.args.scope) as AuraType)

				env.bindings.set("recr", {
					type: { type: "closure", params: paramsTypes, returnType },
					scope: []
					mutable: false,
				})

				return {
					type: "closure",
					params: new Map(
						params.asIteration().children.map((node) => {
							const name = node.child(0).sourceString
							const paramType = node.child(2).type(this.args.scope) as AuraType
							env.bindings.set(name, {
								type: paramType,
								value: { type: "identifier", name, valueType: paramType },
								scope: [...this.args.scope, closureScopeId],
								mutable: false
							})

							return [name, paramType]
						})
					),
					scopeId: closureScopeId,
					returnType: body.typeOf([...this.args.scope, closureScopeId]) as AuraType,
					body: body.expr([...this.args.scope, closureScopeId]) as AuraExpression
				}
			},
			Expression_list(_, elements, _1, _2) {
				return {
					type: "list",
					elements: elements
						.asIteration()
						.children.map((node) => node.expr(this.args.scope) as AuraExpression)
				}
			},
			Expression_map(_, entries, _1, _3) {
				return {
					type: "map",
					entries: new Map(
						entries.asIteration().children.map((node) => [
							node.child(0).expr(this.args.scope) as AuraExpression,
							node.child(2).expr(this.args.scope) as AuraExpression
						])
					)
				}
			},
			Expression_tuple(_, items, _1, _2) {
				return {
					type: "tuple",
					items: items.asIteration().children.map(node =>
						node.expr(this.args.scope) as AuraExpression
					)
				}
			},
			Expression_object(_, fields, _1, _2) {
				return {
					type: "object",
					fields: new Map(
						fields.asIteration().children.map((node) => [
							node.child(0).sourceString,
							node.child(2).expr(this.args.scope) as AuraExpression
						])
					)
				}
			},
			Expression_ident(name) {
				const binding = env.bindings.get(name.sourceString)
				if (!binding) {
					throw new Error(
						`Could not find identifier ${name.sourceString} because it is ${suggestions(
							"does not exist",
							name.sourceString,
							env.bindings.keys()
						)}`
					)
				} else if (this.args.scope?.all((layer: string) => binding.scope?.includes(layer))) {
					throw new Error(
						`Could not find identifier ${name.sourceString} because it is ${
							env.bindings.has(name.sourceString)
								? "out of scope"
								: suggestions("does not exist", name.sourceString, env.bindings.keys())
						}`
					)
				} else {
					return binding.value
				}
			},
			Expression_closure_call(callee, _, _1, args, _2) {
				const calleeValue = callee.expr(this.args.scope) as AuraExpression
				if (calleeValue.type === "closure") {
					const argValues = args.children
					if (argValues.length !== calleeValue.params.size) {
						throw new Error(
							`Expected ${calleeValue.params.size} arguments, but got ${argValues.length}`
						)
					}

					;[...calleeValue.params.entries()].forEach(([name, type], index) => {
						const argValue = argValues[index]
						if (argValue.typeOf(this.args.scope) !== type.type) {
							throw new Error(
								`Expected argument ${index} to be of type ${type.type}, but got ${argValue.type}`
							)
						}

						env.bindings.set(name, {
							type,
							value: argValue.expr(this.args.scope),
							scope: [calleeValue.scopeId],
							mutable: false
						})
					})

					return calleeValue.body
				} else throw new Error(`Cannot call a value of type ${calleeValue.type}`)
			},
			Expression_parens(_, expr, _1) {
				return expr.expr(this.args.scope)
			},
			Expression_binary_op(lhs, op, rhs) {
				const lhsValue: AuraExpression = lhs.expr(this.args.scope)
				const lhsType: AuraType = lhs.typeOf(this.args.scope)
				const rhsValue: AuraExpression = rhs.expr(this.args.scope)
				const rhsType: AuraType = rhs.typeOf(this.args.scope)

				// assignment checks
				{
					// lhs is not an identifier
					if (assignmentOperations.includes(op.sourceString) && lhsValue.type !== "identifier")
						throw new Error(`Cannot assign to a value of type ${typeName(lhsType)}`)

					// lhs is an identifier, but it is immutable
					if (
						assignmentOperations.includes(op.sourceString) &&
						lhsValue.type === "identifier" &&
						env.bindings.get(lhsValue.name)?.mutable === false
					)
						throw new Error(`Cannot assign to ${lhsValue.name} since it is immutable`)

					// the types of the lhs and rhs are different
					if (
						assignmentOperations.includes(op.sourceString) &&
						lhsValue.type === "identifier" &&
						!assignableTo(rhsType, env.bindings.get(lhsValue.name)?.type)
					)
						throw new Error(
							`Cannot assign a value of type ${typeName(rhsType)} to ${
								lhsValue.name
							} of type ${typeName(env.bindings.get(lhsValue.name)!.type)}`
						)
				}

				// boolean operations
				if (
					booleanOperations.includes(op.sourceString) &&
					(lhsType.type !== "boolean" || rhsType.type !== "boolean")
				)
					throw new Error(
						`Cannot use boolean operations on values of type ${typeName(lhsType)} and ${typeName(
							rhsType
						)}`
					)

				// number operations on non-numbers
				if (
					numberOperations.includes(op.sourceString) &&
					(!["int", "float"].includes(lhsType.type) || !["int", "float"].includes(rhsType.type))
				)
					throw new Error(
						`Cannot use number operations on values of type ${typeName(lhsType)} and ${typeName(
							rhsType
						)}`
					)

				// equating two values of different types
				if (equalityOperations.includes(op.sourceString) && !deepEqual(lhsType, rhsType))
					throw new Error(
						`Cannot equate two values of type "${typeName(lhsType)}" and "${typeName(rhsType)}"`
					)

				// comparing two values of different types
				if (comparisonOperations.includes(op.sourceString) && !deepEqual(lhsType, rhsType))
					throw new Error(
						`Cannot compare two values of type "${typeName(lhsType)}" and "${typeName(rhsType)}"`
					)

				const comparableTypes = ["int", "float", "char"]
				// comparing two values of unordered types
				if (
					comparisonOperations.includes(op.sourceString) &&
					(!comparableTypes.includes(lhsType.type) || comparableTypes.includes(rhsType.type))
				)
					throw new Error(
						`Cannot compare two values of type ${typeName(lhsType)} and ${typeName(
							rhsType
						)} since they are unordered`
					)

				// inclusion operations: `in`, `!in`
				{
					const containerTypes = ["string", "list", "map"]
					const containerType =
						rhsValue.type === "identifier" ? env.bindings.get(rhsValue.name)!.type : rhsType
					// inclusion operations on non-containers
					if (
						inclusionOperations.includes(op.sourceString) &&
						!containerTypes.includes(containerType.type)
					)
						throw new Error(`Cannot use ${op.sourceString} on a value of type ${typeName(rhsType)}`)

					// mismatch between item and container item type
					if (
						inclusionOperations.includes(op.sourceString) &&
						((containerType.type === "string" && lhsType.type !== "char") ||
							(containerType.type === "list" && lhsType !== containerType.elementType) ||
							(containerType.type === "map" && lhsType !== containerType.keyType))
					)
						throw new Error(
							`Cannot use ${op.sourceString} on a value of type ${typeName(
								lhsType
							)} and a container of type ${typeName(containerType)}`
						)
				}

				return {
					type: "bin_op",
					left: lhs.expr(this.args.scope) as AuraExpression,
					right: rhs.expr(this.args.scope) as AuraExpression,
					op: op.sourceString as BinaryOp
				}
			},
			Expression_unary_op_prefix(op, expr) {
				const operandType = expr.typeOf(this.args.scope) as AuraType
				if (operandType.type !== "boolean" && op.sourceString === "!") {
					throw new Error(
						`Cannot use ${op.sourceString} on a value of type ${typeName(operandType)}`
					)
				}
				if (operandType.type !== "int" && operandType.type !== "float" && op.sourceString === "-") {
					throw new Error(
						`Cannot use ${op.sourceString} on a value of type ${typeName(operandType)}`
					)
				}

				return {
					type: "unary_op",
					operand: expr.expr(this.args.scope) as AuraExpression,
					op: op.sourceString as UnaryOp
				}
			},
			Expression_unary_op_postfix(expr, op) {
				const operandType = expr.typeOf(this.args.scope) as AuraType
				if (
					operandType.type !== "int" &&
					operandType.type === "float" &&
					(op.sourceString === "++" || op.sourceString === "--")
				) {
					throw new Error(
						`Cannot use ${op.sourceString} on a value of type ${typeName(
							operandType
						)} because it is not a number`
					)
				}

				return {
					type: "unary_op",
					operand: expr.expr(this.args.scope) as AuraExpression,
					op: op.sourceString as UnaryOp
				}
			},
			Expression_code_block(_, exprs, _1) {
				const scopeId = crypto.randomUUID()
				return {
					type: "code_block",
					scopeId,
					body: exprs
						.asIteration()
						.children.map((node) => node.expr((this.args.scope as string[]).concat(scopeId)))
				}
			},
			Expression_for(_, _1, name, _2, iterable, _3, body) {
				const scopeId = crypto.randomUUID()
				return {
					type: "for",
					binding: name.sourceString,
					iterable: iterable.expr(this.args.scope) as AuraExpression,
					scopeId,
					body: body.expr(this.args.scope.concat(scopeId)) as AuraExpression
				}
			},
			Expression_if(_, _1, cond, _2, then, _3, otherwise) {
				return {
					type: "if",
					condition: cond.expr(this.args.scope) as AuraExpression,
					then: then.expr(this.args.scope) as AuraExpression,
					else: otherwise.expr(this.args.scope) as AuraExpression
				}
			},
			Expression_let(_, mutable, name, _1, typeAnn, _2, value) {
				const inferredType = value.typeOf(this.args.scope) as AuraType
				const declaredType = typeAnn.child(0)?.type(this.args.scope) as AuraType | undefined

				if (declaredType && !assignableTo(declaredType, inferredType))
					throw new Error(
						`Expected type "${typeName(declaredType)}" for variable declaration "${
							name.sourceString
						}", but got "${typeName(inferredType)}"`
					)

				env.bindings.set(name.sourceString, {
					type: declaredType ?? inferredType,
					value: value.expr(this.args.scope),
					scope: (this.args.scope as string[]).slice(-1),
					mutable: mutable.sourceString === ""
				})

				return {
					type: "let",
					mutable: mutable.sourceString === "",
					name: name.sourceString,
					value: value.expr(this.args.scope) as AuraExpression,
					typeOf: declaredType ?? inferredType
				}
			},
			Expression_while(_, _1, cond, _2, body) {
				return {
					type: "while",
					condition: cond.expr(this.args.scope) as AuraExpression,
					body: body.expr(this.args.scope) as AuraExpression
				}
			},
			Expression_import(_, path, _2, idents, _4, _5) {
				return {
					type: "import",
					path: path.sourceString.slice(1, -1),
					idents:
				}
			},
			Expression_cast(expr, _, type) {
				return {
					type: "cast",
					value: expr.expr(this.args.scope) as AuraExpression,
					newType: type.type(this.args.scope) as AuraType
				} as AuraExpression
			},
			Expression_type_check(expr, _, targetType) {
				return {
					type: "type_check",
					value: expr.expr(this.args.scope) as AuraExpression,
					targetType: targetType.type(this.args.scope) as AuraType
				}
			},
		} satisfies Actions<"Expression_", AuraExpression>)
		.addOperation<AuraProgram>("program(scope)", {
			Program(declarations) {
				return {
					declarations: declarations.asIteration().children.map((decl) => {
						const expr = decl.expr([]) as AuraExpression
						if (expr.type !== "let")
							throw new Error("Only let-declarations are allowed at the top level.")
						if (expr.name === "main" && expr.typeOf.type !== "closure")
							throw new Error(
								`The name "main" at the top level is reserved for the entry point of the program, but it is of type ${typeName(
									expr.typeOf
								)} instead.`
							)

						return expr
					})
				}
			}
		} satisfies Actions<"Program", AuraProgram>)

	if (result === "type") {
		const matchResult = grammar.match(source, "Type")

		if (matchResult.failed()) throw new Error(matchResult.message)
		return semantics(matchResult).type([]) as Ret extends "type" ? AuraType : never
	} else if (result === "expr") {
		const matchResult = grammar.match(source, "Expression")

		if (matchResult.failed()) throw new Error(matchResult.message)
		return semantics(matchResult).expr([]) as Ret extends "expr" ? AuraExpression : never
	} else {
		const matchResult = grammar.match(source)

		if (matchResult.failed()) throw new Error(matchResult.message)
		return semantics(matchResult).program([]) as Ret extends "expr"
			? never
			: Ret extends "type"
				? never
				: AuraProgram
	}
}
