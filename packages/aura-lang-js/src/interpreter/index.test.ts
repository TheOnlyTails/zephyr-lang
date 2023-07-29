import { describe, expect, test } from "vitest"
import { evaluate, type EvaluatedProgram } from "."

describe("interpreter", () => {
	describe("literals", () => {
		describe("integers", () => {
			test("simple integer", () => {
				expect(evaluate(`let integer = 1`)).toStrictEqual({
					declarations: {
						integer: { type: "int", value: 1 },
					},
				} as EvaluatedProgram)
			})

			test("negative numbers", () => {
				expect(evaluate(`let x = -2`)).toStrictEqual({
					declarations: {
						x: -2,
					},
				})
			})

			test("typed number", () => {
				expect(evaluate(`let x: int = 100`)).toStrictEqual({
					declarations: {
						x: 100,
					},
				})
			})

			test("wrongly typed number", () => {
				expect(() => evaluate(`let x: float = 100`)).toThrowError(
					`Expected type "float" for variable declaration "x", but got "int"`
				)
			})
		})

		describe("booleans", () => {
			test("true", () => {
				expect(evaluate(`let bool = true`)).toStrictEqual({
					declarations: {
						bool: true,
					},
				} as EvaluatedProgram)
			})

			test("false", () => {
				expect(evaluate(`let bool = false`)).toStrictEqual({
					declarations: {
						bool: false,
					},
				} as EvaluatedProgram)
			})

			test("typed boolean", () => {
				expect(evaluate(`let bool: boolean = true`)).toStrictEqual({
					declarations: {
						bool: true,
					},
				})
			})

			test("wrongly typed boolean", () => {
				expect(() => evaluate(`let bool: list<int> = true`)).toThrowError(
					`Expected type "list<int>" for variable declaration "bool", but got "boolean"`
				)
			})
		})

		describe("floating point numbers", () => {
			test("simple fp number", () => {
				expect(evaluate(`let floating = 1.234`)).toStrictEqual({
					declarations: {
						floating: 1.234,
					},
				} as EvaluatedProgram)
			})

			test("many digits of precision", () => {
				expect(evaluate(`let floating = 1.23456789012345678`)).toStrictEqual({
					declarations: {
						floating: 1.23456789012345678,
					},
				})
			})

			test("typed fp number", () => {
				expect(evaluate(`let floating: float = 1.234`)).toStrictEqual({
					declarations: {
						floating: 1.234,
					},
				})
			})

			test("wrongly typed fp number", () => {
				expect(() => evaluate(`let floating: map<int, float> = 1.234`)).toThrowError(
					`Expected type "map<int, float>" for variable declaration "floating", but got "float"`
				)
			})
		})

		describe("strings", () => {
			test("simple string", () => {
				expect(evaluate(`let string = "hello"`)).toStrictEqual({
					declarations: {
						string: "hello",
					},
				} as EvaluatedProgram)
			})

			test("empty string", () => {
				expect(evaluate(`let string = ""`)).toStrictEqual({
					declarations: {
						string: "",
					},
				} as EvaluatedProgram)
			})

			test("unicode", () => {
				expect(evaluate(`let string = "❤️ 🏳️‍⚧️ 👨‍👨‍👧‍👦"`)).toStrictEqual({
					declarations: {
						string: "❤️ 🏳️‍⚧️ 👨‍👨‍👧‍👦",
					},
				} as EvaluatedProgram)
			})

			test("typed string", () => {
				expect(evaluate(`let string: string = "hello"`)).toStrictEqual({
					declarations: {
						string: "hello",
					},
				})
			})

			test("wrongly typed string", () => {
				expect(() => evaluate(`let string: boolean = "hello"`)).toThrowError(
					`Expected type "boolean" for variable declaration "string", but got "string"`
				)
			})
		})

		describe("characters", () => {
			test("simple", () => {
				expect(evaluate(`let character = 'a'`)).toStrictEqual({
					declarations: {
						character: "a",
					},
				} as EvaluatedProgram)
			})

			test("multiple chars error", () => {
				expect(() => evaluate(`let character = 'ab'`)).toThrowError(`Expected "'"`)
			})

			test("unicode", () => {
				expect(evaluate(`let character = '🙂'`)).toStrictEqual({
					declarations: {
						character: "🙂",
					},
				})
			})

			test("typed character", () => {
				expect(evaluate(`let character: char = 'a'`)).toStrictEqual({
					declarations: {
						character: "a",
					},
				})
			})

			test("wrongly typed character", () => {
				expect(() => evaluate(`let character: string = 'a'`)).toThrowError(
					`Expected type "string" for variable declaration "character", but got "char"`
				)
			})
		})

		describe("lists", () => {
			test("simple list", () => {
				expect(evaluate(`let list = #[1, 2, 3]`)).toStrictEqual({
					declarations: {
						list: [1, 2, 3],
					},
				} as EvaluatedProgram)
			})

			test("nested list", () => {
				expect(evaluate(`let list = #[#[1, 2], #[3, 4]]`)).toStrictEqual({
					declarations: {
						list: [
							[1, 2],
							[3, 4],
						],
					},
				})
			})

			test("empty list", () => {
				expect(evaluate(`let list = #[]`)).toStrictEqual({
					declarations: {
						list: [],
					},
				} as EvaluatedProgram)
			})

			test("trailing comma", () => {
				expect(evaluate(`let list = #[1, 2, 3,]`)).toStrictEqual({
					declarations: {
						list: [1, 2, 3],
					},
				} as EvaluatedProgram)
			})

			test("typed list", () => {
				expect(evaluate(`let list: list<int> = #[1, 2, 3]`)).toStrictEqual({
					declarations: {
						list: [1, 2, 3],
					},
				} as EvaluatedProgram)
			})

			test("wrongly typed list", () => {
				expect(() => evaluate(`let list: list<string> = #[1, 2, 3]`)).toThrowError(
					`Expected type "list<string>" for variable declaration "list", but got "list<int>"`
				)
			})
		})

		describe("maps", () => {
			test("simple map", () => {
				expect(evaluate(`let map = #{"a": 1, "b": 2, "c": 3}`)).toStrictEqual({
					declarations: {
						map: new Map(
							Object.entries({
								a: 1,
								b: 2,
								c: 3,
							})
						),
					},
				} as EvaluatedProgram)
			})

			test("nested map", () => {
				expect(evaluate(`let map = #{"a": #{"a": 1}, "b": #{"b": 2}}`)).toStrictEqual({
					declarations: {
						map: new Map(
							Object.entries({
								a: new Map(Object.entries({ a: 1 })),
								b: new Map(Object.entries({ b: 2 })),
							})
						),
					},
				} as EvaluatedProgram)
			})

			test("empty map", () => {
				expect(evaluate(`let map = #{}`)).toStrictEqual({
					declarations: {
						map: new Map(),
					},
				})
			})
		})
	})

	describe("expressions", () => {
		describe("binary operations", () => {
			test("simple binary op", () => {
				expect(evaluate(`let x = 1 + 1`)).toStrictEqual({
					declarations: {
						x: 2,
					},
				} as EvaluatedProgram)
			})

			test("chained binary op", () => {
				expect(evaluate(`let x = 1 + 1 + 1`)).toStrictEqual({
					declarations: {
						x: 3,
					},
				})
			})

			test("order of operations", () => {
				expect(evaluate(`let x = 2 + 5 * 2`)).toStrictEqual({
					declarations: {
						x: 12,
					},
				})
			})

			test("parenthesized order of operations", () => {
				expect(evaluate(`let x = (2 + 5) * 2`)).toStrictEqual({
					declarations: {
						x: 14,
					},
				})
			})

			describe("equality", () => {
				test("simple", () => {
					expect(evaluate(`let x = 1 == 1`)).toStrictEqual({
						declarations: {
							x: true,
						},
					})
				})

				test("different types", () => {
					expect(() => evaluate(`let x = 2 == "string"`)).toThrowError(
						`Cannot equate two values of type "int" and "string"`
					)
				})
			})
		})
	})
})
