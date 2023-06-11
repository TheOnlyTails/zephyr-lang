import { type AuraValue, evalAst } from "."
import type { AuraProgram } from "../parser/ast"
import { auraFileExt } from ".."
import { relative } from "node:path"

export const evalImport = (
	sourcePath: string,
	importPath: string,
	ast: AuraProgram,
	idents: Map<string, string>
): Map<string, AuraValue> => {
	const importedPath = relative(sourcePath, importPath + "." + auraFileExt)
	const result = new Map<string, AuraValue>()

	for (let [name, alias] of idents) {
		const evaluatedDeclarations = evalAst(ast, importedPath).declarations
		result.set(alias, evaluatedDeclarations[name])
	}

	return result
}
