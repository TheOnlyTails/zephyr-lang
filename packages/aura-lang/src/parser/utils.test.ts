import { describe, expect, test } from "vitest"
import { typeName } from "./utils"

describe("typeName", () => {
	test("primitives", () => {
		expect(typeName({ type: "int" })).toBe("int")
		expect(typeName({ type: "float" })).toBe("float")
		expect(typeName({ type: "char" })).toBe("char")
		expect(typeName({ type: "string" })).toBe("string")
		expect(typeName({ type: "boolean" })).toBe("boolean")
		expect(typeName({ type: "void" })).toBe("void")
	})

	test("primitive generics", () => {
		expect(
			typeName({
				type: "list",
				elementType: { type: "int" },
			})
		).toBe("list<int>")

		expect(
			typeName({
				type: "map",
				keyType: { type: "int" },
				valueType: { type: "string" },
			})
		).toBe("map<int, string>")
	})

	test("nested primitive generics", () => {
		expect(
			typeName({ type: "list", elementType: { type: "list", elementType: { type: "int" } } })
		).toBe("list<list<int>>")

		expect(
			typeName({
				type: "map",
				keyType: { type: "list", elementType: { type: "int" } },
				valueType: { type: "string" },
			})
		).toBe("map<list<int>, string>")
	})

	test("closures", () => {
		expect(
			typeName({
				type: "closure",
				params: [{ type: "int" }, { type: "string" }],
				returnType: { type: "boolean" },
			})
		).toBe("(int, string) -> boolean")

		expect(
			typeName({
				type: "closure",
				params: [{ type: "int" }, { type: "string" }],
				returnType: { type: "closure", params: [{ type: "int" }], returnType: { type: "boolean" } },
			})
		).toBe("(int, string) -> (int) -> boolean")
	})

	test("structs", () => {
		expect(
			typeName({
				type: "struct",
				generics: [],
				fields: new Map([
					["a", { type: "int" }],
					["b", { type: "string" }],
				]),
			})
		).toBe(`{\t\na: int,\t\nb: string\n}`)

		expect(
			typeName({
				type: "struct",
				generics: ["T"],
				fields: new Map([
					["a", { type: "int" }],
					["b", { type: "identifier", name: "T" }],
				]),
			})
		).toBe(`<T>{\t\na: int,\t\nb: T\n}`)
	})
})
