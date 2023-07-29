import fuzzyfind from "fuzzyfind"
import type { AuraExpression, AuraType } from "./ast"
import { match, P } from "ts-pattern"
import { deepEqual } from "fast-equals"

export const typeName = (type: AuraType): string => {
	return match(type)
		.with(
			{ type: P.union("int", "float", "char", "string", "boolean", "void") },
			(type) => type.type
		)
		.with(
			{ type: "closure" },
			(type) => `(${type.params.map(typeName).join(", ")}) -> ${typeName(type.returnType)}`
		)
		.with({ type: "list" }, (type) => `list<${typeName(type.elementType)}>`)
		.with({ type: "map" }, (type) => `map<${typeName(type.keyType)}, ${typeName(type.valueType)}>`)
		.with({ type: "union" }, (type) => [...type.types].map(typeName).join(" | "))
		.with(
			{ type: "identifier" },
			(type) =>
				type.name +
				(type.generics && type.generics.length > 0 ? "<" + type.generics.join(", ") + ">" : "")
		)
		.with(
			{ type: "object" },
			(type) =>
				"{" +
				[...type.fields.entries()].map(([name, type]) => `${name}: ${typeName(type)}`).join(",") +
				"}"
		)
		.exhaustive()
}

export const assignableTo = (
	from: AuraType | undefined,
	to: AuraType | undefined
): true | Error => {
	if (!from) return new Error(`unreachable: from was undefined`)
	if (!to) return new Error(`unreachable: to was undefined`)

	return (
		match<[AuraType | undefined, AuraType | undefined], true | Error>([from, to])
			// T : T
			.when(
				([from, to]) => deepEqual(from, to),
				(_) => true
			)
			// floats are assignable to ints by flooring
			.with(
				P.union([{ type: "float" }, { type: "int" }], [{ type: "int" }, { type: "float" }]),
				(_) => true
			)
			// list<char> <-> string
			.with(
				P.union(
					[{ type: "string" }, { type: "list", elementType: { type: "char" } }],
					[{ type: "list", elementType: { type: "char" } }, { type: "string" }]
				),
				(_) => true
			)
			// `A | B` is assignable to `A | B | C`.
			.with([{ type: "union" }, { type: "union" }], ([from, to]) =>
				from.types.every((fromMember) =>
					to.types.some((toMember) => assignableTo(fromMember, toMember))
				)
					? true
					: new Error(
							`The union ${typeName(from)} is not assignable to ${typeName(
								to
							)} because it is not fully contained within it.`
					  )
			)
			// A, B, and C are assignable to `A | B | C`.
			.when(
				([from, to]) =>
					from?.type === "union" && from.types.some((member) => assignableTo(to, member)),
				(_) => true
			)
			// an object where all the values are assignable to the value type of
			// a string-keyed map is assignable to that map.
			.with([{ type: "object" }, { type: "map" }], ([from, to]) =>
				assignableTo(to.keyType , { type: "string" })
					? [...from.fields.values()].every((field) => assignableTo(field, to.valueType))
						? true
						: new Error(
								`Type ${typeName(from)} is not assignable to ${typeName(to)} because ${typeName({
									type: "union",
									types: [...from.fields.values()],
								})} is not assignable to ${typeName(to.valueType)}`
						  )
					: new Error(
							`Type ${typeName(from)} is not assignable to ${typeName(to)} because ${typeName(
								to.keyType
							)} is not assignable to ${typeName({ type: "string" })}`
					  )
			)
			// a string-keyed map whose value type is assignable to
			// all of an object's value types is assignable to that object.
			.with([{ type: "map", keyType: { type: "string" } }, { type: "object" }], ([from, to]) =>
				[...to.fields.values()].every((objVal) => assignableTo(from.valueType, objVal))
					? true
					: new Error(
							`Type ${typeName(from)} is not assignable to ${typeName(
								to
							)} because its value type is not assignable to ${typeName({
								type: "union",
								types: [...to.fields.values()],
							})}`
					  )
			)
			.otherwise(
				([from, to]) => new Error(`Type ${typeName(from!)} is not assignable to ${typeName(to!)}`)
			)
	)
}

export const suggestions = (msg: string, wanted: string, options: Iterable<string>) => {
	const alternatives = fuzzyfind(wanted, [...options]).at(0)
	const didYouMean = alternatives ? ` Did you mean ${alternatives}?` : ""

	return msg + didYouMean
}
