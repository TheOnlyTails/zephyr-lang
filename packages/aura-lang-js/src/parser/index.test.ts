import { parse } from "./index.ts"
import { describe, expect, test } from "vitest"
import type { AuraExpression, AuraType } from "./ast.ts"
import { deepEqual } from "fast-equals"

describe("constant literals", () => {
	test("int", async () => {
		expect(parse("1", "expr")).toStrictEqual({
			type: "int",
			value: 1,
		} as AuraExpression)

		expect(parse("123", "expr")).toStrictEqual({
			type: "int",
			value: 123,
		} as AuraExpression)
	})

	test("float", async () => {
		expect(parse("1.0", "expr")).toStrictEqual({
			type: "float",
			value: 1.0,
		} as AuraExpression)

		expect(parse("123.456", "expr")).toStrictEqual({
			type: "float",
			value: 123.456,
		} as AuraExpression)

		expect(() => parse("123.", "expr")).toThrowError("Expected a digit")
	})

	test("char", async () => {
		expect(parse(`'a'`, "expr")).toStrictEqual({
			type: "char",
			value: "a",
		} as AuraExpression)

		expect(parse(`'\\n'`, "expr")).toStrictEqual({
			type: "char",
			value: "\n",
		} as AuraExpression)

		expect(parse(`'\\\''`, "expr")).toStrictEqual({
			type: "char",
			value: "'",
		} as AuraExpression)

		expect(parse(`'\\\\'`, "expr")).toStrictEqual({
			type: "char",
			value: `\\`,
		} as AuraExpression)

		expect(parse(`'\\u2764\\uFE0F'`, "expr")).toStrictEqual({
			type: "char",
			value: "❤️",
		} as AuraExpression)

		expect(parse(`'😆'`, "expr")).toStrictEqual({
			type: "char",
			value: "😆",
		} as AuraExpression)
	})

	test("string", async () => {
		expect(parse(`"hello"`, "expr")).toStrictEqual({
			type: "string",
			value: "hello",
		} as AuraExpression)

		expect(parse(`"hello\\nworld"`, "expr")).toStrictEqual({
			type: "string",
			value: "hello\nworld",
		} as AuraExpression)

		expect(parse(`"hello\\u2764\\uFE0Fworld"`, "expr")).toStrictEqual({
			type: "string",
			value: "hello❤️world",
		} as AuraExpression)

		expect(parse(`"hello😆world"`, "expr")).toStrictEqual({
			type: "string",
			value: "hello😆world",
		} as AuraExpression)
	})

	test("boolean", async () => {
		expect(parse(`true`, "expr")).toStrictEqual({
			type: "boolean",
			value: true,
		} as AuraExpression)

		expect(parse(`false`, "expr")).toStrictEqual({
			type: "boolean",
			value: false,
		} as AuraExpression)
	})

	test("list literals", async () => {
		expect(parse(`#[]`, "expr")).toStrictEqual({
			type: "list",
			elements: [],
		} as AuraExpression)

		expect(parse(`#[1]`, "expr")).toStrictEqual({
			type: "list",
			elements: [
				{
					type: "int",
					value: 1,
				},
			],
		} as AuraExpression)

		expect(parse(`#[1, 2]`, "expr")).toStrictEqual({
			type: "list",
			elements: [
				{
					type: "int",
					value: 1,
				},
				{
					type: "int",
					value: 2,
				},
			],
		} as AuraExpression)

		expect(parse(`#[1, 2, 3]`, "expr")).toStrictEqual({
			type: "list",
			elements: [
				{
					type: "int",
					value: 1,
				},
				{
					type: "int",
					value: 2,
				},
				{
					type: "int",
					value: 3,
				},
			],
		} as AuraExpression)

		expect(parse(`#[1, 2, 3,]`, "expr")).toStrictEqual({
			type: "list",
			elements: [
				{
					type: "int",
					value: 1,
				},
				{
					type: "int",
					value: 2,
				},
				{
					type: "int",
					value: 3,
				},
			],
		} as AuraExpression)
	})

	test("map literals", async () => {
		expect(parse(`#{}`, "expr")).toStrictEqual({
			type: "map",
			entries: new Map(),
		} as AuraExpression)

		expect(parse(`#{1: 2}`, "expr")).toStrictEqual({
			type: "map",
			entries: new Map([
				[
					{ type: "int", value: 1 },
					{ type: "int", value: 2 },
				],
			]),
		} as AuraExpression)

		expect(parse(`#{1: 2, 3: 4}`, "expr")).toStrictEqual({
			type: "map",
			entries: new Map([
				[
					{ type: "int", value: 1 },
					{ type: "int", value: 2 },
				],
				[
					{ type: "int", value: 3 },
					{ type: "int", value: 4 },
				],
			]),
		} as AuraExpression)

		expect(parse(`#{1: 2, 3: 4,}`, "expr")).toStrictEqual({
			type: "map",
			entries: new Map([
				[
					{ type: "int", value: 1 },
					{ type: "int", value: 2 },
				],
				[
					{ type: "int", value: 3 },
					{ type: "int", value: 4 },
				],
			]),
		})
	})

	test("closure literals", async () => {
		expect(parse(`() -> {}`, "expr")).toSatisfy(
			(expr: AuraExpression) =>
				expr.type === "closure" &&
				expr.params.size === 0 &&
				expr.returnType.type === "void" &&
				expr.body.type === "code_block"
		)

		expect(parse(`(a: int) -> {}`, "expr")).toSatisfy(
			(expr: AuraExpression) =>
				expr.type === "closure" &&
				deepEqual(expr.params, new Map([["a", { type: "int" }]])) &&
				expr.returnType.type === "void" &&
				expr.body.type === "code_block" &&
				expr.body.body.length === 0
		)
	})
})

describe("types", async () => {
	test("literals", async () => {
		expect(parse(`int`, "type")).toStrictEqual({
			type: "int",
		} as AuraType)

		expect(parse(`float`, "type")).toStrictEqual({
			type: "float",
		} as AuraType)

		expect(parse(`char`, "type")).toStrictEqual({
			type: "char",
		} as AuraType)

		expect(parse(`string`, "type")).toStrictEqual({
			type: "string",
		} as AuraType)

		expect(parse(`boolean`, "type")).toStrictEqual({
			type: "boolean",
		} as AuraType)

		expect(parse(`void`, "type")).toStrictEqual({
			type: "void",
		} as AuraType)
	})

	test("list", async () => {
		expect(parse(`list<int>`, "type")).toStrictEqual({
			type: "list",
			elementType: {
				type: "int",
			},
		} as AuraType)

		expect(parse(`list<list<int>>`, "type")).toStrictEqual({
			type: "list",
			elementType: {
				type: "list",
				elementType: {
					type: "int",
				},
			},
		} as AuraType)
	})

	test("map", async () => {
		expect(parse(`map<int, string>`, "type")).toStrictEqual({
			type: "map",
			keyType: {
				type: "int",
			},
			valueType: {
				type: "string",
			},
		} as AuraType)

		expect(parse(`map<int, map<int, string>>`, "type")).toStrictEqual({
			type: "map",
			keyType: {
				type: "int",
			},
			valueType: {
				type: "map",
				keyType: {
					type: "int",
				},
				valueType: {
					type: "string",
				},
			},
		} as AuraType)
	})

	test("closure", async () => {
		expect(parse(`() -> void`, "type")).toStrictEqual({
			type: "closure",
			params: [],
			returnType: {
				type: "void",
			},
		} as AuraType)

		expect(parse(`(int) -> void`, "type")).toStrictEqual({
			type: "closure",
			params: [{ type: "int" }],
			returnType: {
				type: "void",
			},
		} as AuraType)

		expect(parse(`(int, string) -> void`, "type")).toStrictEqual({
			type: "closure",
			params: [{ type: "int" }, { type: "string" }],
			returnType: {
				type: "void",
			},
		} as AuraType)

		expect(parse(`(int, string) -> int`, "type")).toStrictEqual({
			type: "closure",
			params: [{ type: "int" }, { type: "string" }],
			returnType: {
				type: "int",
			},
		} as AuraType)

		expect(parse(`(int, string) -> list<int>`, "type")).toStrictEqual({
			type: "closure",
			params: [{ type: "int" }, { type: "string" }],
			returnType: {
				type: "list",
				elementType: {
					type: "int",
				},
			},
		} as AuraType)
	})

	test("unions", async () => {
		expect(parse(`int | string`, "type")).toStrictEqual({
			type: "union",
			types: [
				{
					type: "int",
				},
				{
					type: "string",
				},
			],
		} as AuraType)

		expect(parse(`int | string | boolean`, "type")).toStrictEqual({
			type: "union",
			types: [
				{
					type: "int",
				},
				{
					type: "string",
				},
				{
					type: "boolean",
				},
			],
		} as AuraType)
	})
})
