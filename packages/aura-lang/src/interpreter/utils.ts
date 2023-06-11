import { P, match } from "ts-pattern"
import type { AuraValue } from "."
import type { AuraType } from "../parser/ast"

export const valueType = (value: AuraValue): AuraType => {
	return match<AuraValue, AuraType>(value)
		.with({ type: P.union("int", "float", "string", "char", "boolean") }, (val) => ({
			type: val.type,
		}))
		.with({ type: "list" }, (value) => {
			const first = value.value.at(0)
			return {
				type: "list",
				elementType: first ? valueType(first) : { type: "void" },
			}
		})
		.with({ type: "map" }, (value) => {
			const first = [...value.value.entries()].at(0)
			return {
				type: "map",
				keyType: first ? valueType(first[0]) : { type: "void" },
				valueType: first ? valueType(first[1]) : { type: "void" },
			}
		})
		.with({ type: "object" }, (value) => ({
			type: "object",
			fields: new Map(
				Object.entries(value.value).map(([key, fieldValue]) => [key, valueType(fieldValue)])
			),
		}))
		.with({ type: "closure" }, (value) => ({
			type: "closure",
			params: [...value.params.values()],
			returnType: value.returnType,
		}))
		.with({ type: "void" }, () => ({ type: "void" }))
		.exhaustive()
}
