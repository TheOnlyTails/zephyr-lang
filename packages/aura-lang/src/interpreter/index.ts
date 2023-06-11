import { parse } from "../parser"
import type { AuraProgram, AuraType } from "../parser/ast"
import { evalExpr } from "./expr"
import { findUpSync } from "find-up"
import path from "node:path"
import { type AuraOptions, defaultOptions } from ".."

export type InterpreterEnvironment = Record<string, AuraValue>
export type AuraValue =
	| { type: "int"; value: number }
	| { type: "float"; value: number }
	| { type: "char"; value: string }
	| { type: "string"; value: string }
	| { type: "boolean"; value: boolean }
	| { type: "void" } // void
	| { type: "list"; value: AuraValue[] } // list
	| { type: "map"; value: Map<AuraValue, AuraValue> } // map
	| { type: "object"; value: { [name: string]: AuraValue } } // object
	| {
			type: "closure"
			params: Map<string, AuraType>
			returnType: AuraType
			value: (...params: AuraValue[]) => AuraValue
	  } // closure

export type EvaluatedProgram = {
	declarations: Record<string, AuraValue>
}

export const evaluate = (source: string, opts: AuraOptions = {}) =>
	evalAst(parse(source), { ...defaultOptions, ...opts })

export const evalAst = (source: AuraProgram, opts: AuraOptions): EvaluatedProgram => {
	const { src = "src", config = "aura.toml" } = opts

	const root = findUpSync(config)
	if (!root)
		throw new Error(`Could not find the project root with a config file named "${config}".`)
	const basePath = path.resolve(root, src)

	const env: InterpreterEnvironment = {}
	const declarations = new Map(
		source.declarations.map(({ name, value }) => [name, evalExpr(value, env, basePath)])
	)

	const mainFunc = declarations.get("main")

	if (mainFunc) {
		if (typeof mainFunc !== "function")
			throw new Error(
				`The name "main" is reserved for the main function in top-level declarations, which is the entry point of the program`
			)
		if (mainFunc.arguments.length !== 0)
			throw new Error(`The main function must not take any parameters`)

		const programResult = mainFunc()
		if (programResult) console.warn(`Any values returned by the main function are thrown away.`)
	}

	return {
		declarations: Object.fromEntries(declarations.entries()),
	}
}
