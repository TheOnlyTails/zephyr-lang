export type AuraProgram = {
	declarations: Extract<AuraExpression, { type: "let" }>[]
}

export type AuraExpression =
	| { type: "int"; value: number }
	| { type: "float"; value: number }
	| { type: "char"; value: string }
	| { type: "string"; value: string }
	| { type: "boolean"; value: boolean }
	| { type: "list"; elements: AuraExpression[] }
	| { type: "map"; entries: Map<AuraExpression, AuraExpression> }
	| { type: "tuple"; items: AuraExpression[] }
	| { type: "object"; fields: Map<string, AuraExpression> }
	| {
			type: "closure"
			params: Map<string, AuraType>
			returnType: AuraType
			body: AuraExpression
			scopeId: `${string}-${string}-${string}-${string}-${string}`
	  }
	| { type: "identifier"; name: string; valueType: AuraType }
	| { type: "call"; func: AuraExpression; args: AuraExpression[] }
	| { type: "bin_op"; op: BinaryOp; left: AuraExpression; right: AuraExpression }
	| { type: "unary_op"; op: UnaryOp; operand: AuraExpression }
	| { type: "let"; name: string; mutable: boolean; value: AuraExpression; typeOf: AuraType }
	| {
			type: "if"
			condition: AuraExpression
			then: AuraExpression
			else: AuraExpression
	  }
	| { type: "while"; condition: AuraExpression; body: AuraExpression }
	| {
			type: "for"
			binding: string
			iterable: AuraExpression
			body: AuraExpression
			scopeId: `${string}-${string}-${string}-${string}-${string}`
	  }
	| {
			type: "import"
			path: string
			idents?: Map<string, string | undefined>
	  }
	| {
			type: "cast"
			value: AuraExpression
			newType: AuraType
	  }
	| {
			type: "type_check"
			value: AuraExpression
			targetType: AuraType
	  }
	| {
			type: "code_block"
			body: AuraExpression[]
			scopeId: `${string}-${string}-${string}-${string}-${string}`
	  }

export type BinaryOp = keyof typeof binaryOpResult
export type UnaryOp = keyof typeof unaryOpResult

export const assignmentOperations = ["=", "+=", "-=", "*=", "/=", "%=", "**="]
export const booleanOperations = ["&&", "||"]
export const numberOperations = ["+", "-", "*", "/", "%", "**"]
export const equalityOperations = ["==", "!="]
export const comparisonOperations = ["<", "<=", ">", ">="]
export const inclusionOperations = ["in", "!in"]

export const binaryOpResult = {
	"=": (lhs, _) => lhs,
	"+": (lhs, _) => lhs,
	"+=": (lhs, _) => lhs,
	"-": (lhs, _) => lhs,
	"-=": (lhs, _) => lhs,
	"*": (lhs, _) => lhs,
	"*=": (lhs, _) => lhs,
	"/": (_, _1) => ({ type: "float" }),
	"/=": (_, _1) => ({ type: "float" }),
	"%": (_, _1) => ({ type: "int" }),
	"%=": (_, _1) => ({ type: "int" }),
	"**": (_, _1) => ({ type: "float" }),
	"**=": (_, _1) => ({ type: "float" }),
	"==": (_, _1) => ({ type: "boolean" }),
	"!=": (_, _1) => ({ type: "boolean" }),
	"<": (_, _1) => ({ type: "boolean" }),
	"<=": (_, _1) => ({ type: "boolean" }),
	">": (_, _1) => ({ type: "boolean" }),
	">=": (_, _1) => ({ type: "boolean" }),
	"&&": (_, _1) => ({ type: "boolean" }),
	"||": (_, _1) => ({ type: "boolean" }),
	in: (_, _1) => ({ type: "boolean" }),
	"!in": (_, _1) => ({ type: "boolean" }),
} as const satisfies Record<string, (lhs: AuraType, rhs: AuraType) => AuraType>

export const unaryOpResult = {
	"-": (operand) => operand,
	"!": (_) => ({ type: "boolean" }),
	"++": (operand) => operand,
	"--": (operand) => operand,
} as const satisfies Record<string, (operand: AuraType) => AuraType>

export type AuraType =
	| { type: "int" }
	| { type: "float" }
	| { type: "char" }
	| { type: "string" }
	| { type: "boolean" }
	| { type: "void" }
	| {
			type: "closure"
			params: AuraType[]
			returnType: AuraType
	  }
	| {
			type: "list"
			elementType: AuraType
	  }
	| {
			type: "map"
			keyType: AuraType
			valueType: AuraType
	  }
	| {
			type: "union"
			types: AuraType[]
	  }
	| {
			type: "identifier"
			name: string
			generics?: string[]
	  }
	| {
			type: "object"
			fields: Map<string, AuraType>
	  }
	| {
			type: "tuple"
			items: AuraType[]
	  }
