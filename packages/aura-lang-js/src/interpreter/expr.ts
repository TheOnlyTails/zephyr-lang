import type { AuraExpression, AuraType, BinaryOp, UnaryOp } from "../parser/ast"
import type { AuraValue, InterpreterEnvironment } from "."
import { deepEqual } from "fast-equals"
import { isNativeError } from "node:util/types"
import { match, P } from "ts-pattern"
import { assignableTo } from "../parser/utils"
import { valueType } from "./utils"

export const evalExpr = (
	expr: AuraExpression,
	env: InterpreterEnvironment,
	path: string
): AuraValue => {
	const result = match(expr)
		.returnType<AuraValue | Error>()
		.with({ type: P.union("int", "float", "char", "string", "boolean") }, (expr) => expr)
		.with({ type: "list" }, (expr) => ({
			type: "list",
			value: expr.elements.map((e) => evalExpr(e, env, path)),
		}))
		.with({ type: "map" }, (expr) => ({
			type: "map",
			value: new Map(
				[...expr.entries.entries()].map(([k, v]) => [
					evalExpr(k, env, path),
					evalExpr(v, env, path),
				])
			),
		}))
		.with({ type: "tuple" }, (expr) => ({
			type: "tuple",
			value: expr.items.map((expr) => evalExpr(expr, env, path)),
		}))
		.with({ type: "object" }, (expr) => ({
			type: "object",
			value: Object.fromEntries(
				[...expr.fields.entries()].map(([name, value]) => [name, evalExpr(value, env, path)])
			),
		}))
		.with({ type: "closure" }, (expr) => ({
			type: "closure",
			params: expr.params,
			returnType: expr.returnType,
			value: (...params: AuraValue[]) => {
				const paramsEnv = Object.fromEntries(
					[...expr.params.keys()].map((name, i) => [name, params[i]])
				)
				return evalExpr(expr.body, { ...env, ...paramsEnv }, path)
			},
		}))
		.with(
			{ type: "identifier" },
			(expr) => Object.entries(env).find(([name]) => name === expr.name)![1]
		)
		.with({ type: "call" }, (expr) => {
			const callee = evalExpr(expr.func, env, path)
			return callee.type === "closure"
				? callee.value(...expr.args.map((p) => evalExpr(p, env, path)))
				: Error("Unexpected runtime error: tried to call non-closure value")
		})
		.with({ type: "bin_op" }, (expr) => evalBinOp(expr, env, path))
		.with({ type: "unary_op" }, (expr) => evalUnaryOp(expr, env, path))
		.with({ type: "let" }, (expr) => (env[expr.name] = evalExpr(expr.value, env, path)))
		.with({ type: "if" }, (expr) => {
			const condition = evalExpr(expr.condition, env, path)
			return condition.type === "boolean"
				? evalExpr(condition.value ? expr.then : expr.else, env, path)
				: Error("An if-expression condition must be a boolean")
		})
		.with({ type: "while" }, (expr) => {
			const condition = evalExpr(expr.condition, env, path)
			if (condition.type === "boolean") {
				while (condition) evalExpr(expr.body, env, path)
				return { type: "void" }
			} else return Error()
		})
		.with({ type: "for" }, (expr) => {
			const iterable = evalExpr(expr.iterable, env, path)
			if (iterable.type === "string" || iterable.type === "map" || iterable.type === "list") {
				const collection = match<
					string | AuraValue[] | Map<AuraValue, AuraValue>,
					Iterable<AuraValue>
				>(iterable.value)
					.with(P.string, (value) => [...value].map((c) => ({ type: "char", value: c })))
					.with(P.array(), (value) => value)
					.with(P.map(), (value) => value.keys())
					.exhaustive()

				for (let binding of collection) {
					evalExpr(
						expr.body,
						Object.fromEntries([[expr.binding, binding], ...Object.entries(env)]),
						path
					)
				}
				return { type: "void" }
			} else throw Error()
		})
		.with({ type: "import" }, (expr) => {
			expr.path
		})
		.with({ type: "cast" }, (expr) => {
			const value = evalExpr(expr.value, env, path)

			const result = match<[AuraValue, AuraType], AuraValue | Error>([value, expr.newType])
				.when(
					([value, newType]) => deepEqual(value, newType),
					([value]) => value
				)
				.with([{ type: "float" }, { type: "int" }], ([value]) => ({
					type: "int",
					value: value.value > 0 ? Math.floor(value.value) : Math.ceil(value.value),
				}))
				.with([{ type: "int" }, { type: "float" }], ([value]) => ({
					type: "float",
					value: value.value,
				}))
				.with([{ type: "string" }, { type: "list", elementType: { type: "char" } }], ([value]) => ({
					type: "list",
					value: [...value.value].map((c) => ({ type: "char", value: c })),
				}))
				.with(
					[{ type: "list", value: P.array({ type: "char" }) }, { type: "string" }],
					([value]) => ({
						type: "string",
						value: value.value.map((c) => c.value).join(""),
					})
				)
				.with([{ type: "object" }, { type: "map", keyType: { type: "string" } }], ([value]) => ({
					type: "map",
					value: new Map(
						Object.entries(value.value).map(([key, val]) => [{ type: "string", value: key }, val])
					),
				}))
				.with(
					[{ type: "map", value: P.map({ type: "string" }, P._) }, { type: "object" }],
					([value]) => ({
						type: "object",
						value: Object.fromEntries(value.value.entries()),
					})
				)
				.otherwise(() => Error("unreachable: invalid cast, should be caught at parsing"))

			if (isNativeError(result)) throw result
			return result
		})
		.with({ type: "type_check" }, (expr) => ({
			type: "boolean",
			value: !isNativeError(
				assignableTo(valueType(evalExpr(expr.value, env, path)), expr.targetType)
			),
		}))
		.with({ type: "code_block" }, (expr) => {
			let result: AuraValue | undefined
			for (let line of expr.body) {
				result = evalExpr(line, env, path)
			}
			if (result) return result
			return { type: "void" }
		})
		.exhaustive()

	if (isNativeError(result)) throw result
	return result
}

const evalBinOp = (
	expr: Extract<AuraExpression, { type: "bin_op" }>,
	env: InterpreterEnvironment,
	path: string
): AuraValue => {
	const left = evalExpr(expr.left, env, path)
	const right = evalExpr(expr.right, env, path)

	const result = match<BinaryOp, AuraValue | Error>(expr.op)
		.with("=", () => (expr.left.type === "identifier" ? (env[expr.left.name] = right) : Error()))
		.with("+", () =>
			(left.type === "int" || left.type === "float") &&
			(right.type === "int" || right.type === "float")
				? { type: left.type, value: left.value + right.value }
				: Error()
		)
		.with("+=", () =>
			expr.left.type === "identifier" &&
			(left.type === "int" || left.type === "float") &&
			(right.type === "int" || right.type === "float")
				? (env[expr.left.name] = { type: left.type, value: left.value + right.value })
				: Error()
		)
		.with("-", () =>
			(left.type === "int" || left.type === "float") &&
			(right.type === "int" || right.type === "float")
				? { type: left.type, value: left.value - right.value }
				: Error()
		)
		.with("-=", () =>
			expr.left.type === "identifier" &&
			(left.type === "int" || left.type === "float") &&
			(right.type === "int" || right.type === "float")
				? (env[expr.left.name] = { type: left.type, value: left.value - right.value })
				: Error()
		)
		.with("*", () =>
			(left.type === "int" || left.type === "float") &&
			(right.type === "int" || right.type === "float")
				? { type: left.type, value: left.value * right.value }
				: Error()
		)
		.with("*=", () =>
			expr.left.type === "identifier" &&
			(left.type === "int" || left.type === "float") &&
			(right.type === "int" || right.type === "float")
				? (env[expr.left.name] = { type: left.type, value: left.value * right.value })
				: Error()
		)
		.with("/", () =>
			(left.type === "int" || left.type === "float") &&
			(right.type === "int" || right.type === "float")
				? { type: left.type, value: left.value / right.value }
				: Error()
		)
		.with("/=", () =>
			expr.left.type === "identifier" &&
			(left.type === "int" || left.type === "float") &&
			(right.type === "int" || right.type === "float")
				? (env[expr.left.name] = { type: left.type, value: left.value / right.value })
				: Error()
		)
		.with("%", () =>
			(left.type === "int" || left.type === "float") &&
			(right.type === "int" || right.type === "float")
				? { type: left.type, value: left.value % right.value }
				: Error()
		)
		.with("%=", () =>
			expr.left.type === "identifier" &&
			(left.type === "int" || left.type === "float") &&
			(right.type === "int" || right.type === "float")
				? (env[expr.left.name] = { type: left.type, value: left.value % right.value })
				: Error()
		)
		.with("**", () =>
			(left.type === "int" || left.type === "float") &&
			(right.type === "int" || right.type === "float")
				? { type: left.type, value: left.value }
				: Error()
		)
		.with("**=", () =>
			expr.left.type === "identifier" &&
			(left.type === "int" || left.type === "float") &&
			(right.type === "int" || right.type === "float")
				? (env[expr.left.name] = { type: left.type, value: left.value ** right.value })
				: Error()
		)
		.with("==", () => ({ type: "boolean", value: deepEqual(left, right) }))
		.with("!=", () => ({ type: "boolean", value: !deepEqual(left, right) }))
		.with("<", () => {
			if (
				(left.type === "int" || left.type === "float") &&
				(right.type === "int" || right.type === "float")
			)
				return { type: "boolean", value: left.value < right.value }
			if (left.type === "char" && right.type === "char")
				return { type: "boolean", value: left.value.codePointAt(0)! < right.value.codePointAt(0)! }
			return Error()
		})
		.with("<=", () => {
			if (
				(left.type === "int" || left.type === "float") &&
				(right.type === "int" || right.type === "float")
			)
				return { type: "boolean", value: left.value <= right.value }
			if (left.type === "char" && right.type === "char")
				return { type: "boolean", value: left.value.codePointAt(0)! <= right.value.codePointAt(0)! }
			return Error()
		})
		.with(">", () => {
			if (
				(left.type === "int" || left.type === "float") &&
				(right.type === "int" || right.type === "float")
			)
				return { type: "boolean", value: left.value > right.value }
			if (left.type === "char" && right.type === "char")
				return { type: "boolean", value: left.value.codePointAt(0)! > right.value.codePointAt(0)! }
			return Error()
		})
		.with(">=", () => {
			if (
				(left.type === "int" || left.type === "float") &&
				(right.type === "int" || right.type === "float")
			)
				return { type: "boolean", value: left.value >= right.value }
			if (left.type === "char" && right.type === "char")
				return { type: "boolean", value: left.value.codePointAt(0)! >= right.value.codePointAt(0)! }
			return Error()
		})
		.with("&&", () =>
			left.type === "boolean" && right.type === "boolean"
				? {
						type: "boolean",
						value: left.value && right.value,
				  }
				: Error()
		)
		.with("||", () =>
			left.type === "boolean" && right.type === "boolean"
				? {
						type: "boolean",
						value: left.value || right.value,
				  }
				: Error()
		)
		.with("in", () => {
			if (right.type === "list") return { type: "boolean", value: right.value.includes(left) }
			if (right.type === "map")
				return { type: "boolean", value: [...right.value.keys()].includes(left) }
			if (right.type === "string" && left.type === "char")
				return { type: "boolean", value: right.value.includes(left.value) }

			return Error()
		})
		.with("!in", () => {
			if (right.type === "list") return { type: "boolean", value: !right.value.includes(left) }
			if (right.type === "map")
				return { type: "boolean", value: ![...right.value.keys()].includes(left) }
			if (right.type === "string" && left.type === "char")
				return { type: "boolean", value: !right.value.includes(left.value) }

			return Error()
		})
		.exhaustive()

	if (isNativeError(result)) throw result
	return result
}

const evalUnaryOp = (
	expr: Extract<AuraExpression, { type: "unary_op" }>,
	env: InterpreterEnvironment,
	path: string
): AuraValue => {
	const operand = evalExpr(expr.operand, env, path)

	const result = match<UnaryOp, AuraValue | Error>(expr.op)
		.with("++", () =>
			expr.operand.type === "identifier" && (operand.type === "int" || operand.type === "float")
				? (env[expr.operand.name] = { type: operand.type, value: operand.value + 1 })
				: Error()
		)
		.with("--", () =>
			expr.operand.type === "identifier" && (operand.type === "int" || operand.type === "float")
				? (env[expr.operand.name] = { type: operand.type, value: operand.value - 1 })
				: Error()
		)
		.with("-", () =>
			operand.type === "int" || operand.type === "float"
				? {
						type: operand.type,
						value: -operand.value,
				  }
				: Error()
		)
		.with("!", () =>
			operand.type === "boolean"
				? {
						type: "boolean",
						value: !operand.value,
				  }
				: Error()
		)
		.exhaustive()

	if (isNativeError(result)) throw result
	return result
}
