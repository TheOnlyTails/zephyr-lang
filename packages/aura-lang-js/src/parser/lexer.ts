import { Lexer, createToken } from "chevrotain"

export const NEWLINE = createToken({
	name: "newline",
	pattern: /\n+/,
	group: Lexer.SKIPPED,
})

export const WHITESPACE = createToken({
	name: "whitespace",
	pattern: /\s+/,
	group: Lexer.SKIPPED,
})

export const IDENTIFIER = createToken({
	name: "identifier",
	pattern: /[\p{XID_Start}_]\p{XID_Continue}*/u,
})

export const ANY = createToken({ name: "any", pattern: /./ })
export const DIGIT = createToken({ name: "digit", pattern: /\d+/ })
export const HEX_DIGIT = createToken({ name: "hex_digit", pattern: /[0-1a-fA-F]/ })

export const COMMA = createToken({ name: "comma", pattern: /,/ })
export const PERIOD = createToken({ name: "period", pattern: /\./ })
export const LEFT_PAREN = createToken({ name: "left_paren", pattern: /\(/ })
export const RIGHT_PAREN = createToken({ name: "right_paren", pattern: /\)/ })
export const LEFT_BRACKET = createToken({ name: "left_bracket", pattern: /\[/ })
export const RIGHT_BRACKET = createToken({ name: "right_bracket", pattern: /\]/ })
export const LEFT_BRACE = createToken({ name: "left_brace", pattern: /\{/ })
export const RIGHT_BRACE = createToken({ name: "right_brace", pattern: /\}/ })
export const LEFT_ANGLED = createToken({ name: "left_angled", pattern: /</ })
export const RIGHT_ANGLED = createToken({ name: "right_angled", pattern: />/ })
export const HASH = createToken({ name: "hash", pattern: /#/ })
export const AT_SIGN = createToken({ name: "at_sign", pattern: /@/ })
export const PIPE = createToken({ name: "pipe", pattern: /\|/ })
export const QUOTE = createToken({ name: "quote", pattern: /"/ })
export const APOSTROPHE = createToken({ name: "apostrophe", pattern: /'/ })
export const BACKTICK = createToken({ name: "backtick", pattern: /`/ })
export const COLON = createToken({ name: "colon", pattern: /:/ })
export const SEMICOLON = createToken({ name: "semicolon", pattern: /;/ })
export const UNDERSCORE = createToken({ name: "underscore", pattern: /_/ })
export const SLASH = createToken({ name: "slash", pattern: /\// })
export const BACKSLASH = createToken({ name: "backslash", pattern: /\\/ })
export const TILDE = createToken({ name: "tilde", pattern: /~/ })

// keywords
export const LET = createToken({ name: "let", pattern: /let/ })
export const MUT = createToken({ name: "mut", pattern: /mut/ })
export const IF = createToken({ name: "if", pattern: /if/ })
export const ELSE = createToken({ name: "else", pattern: /else/ })
export const WHILE = createToken({ name: "while", pattern: /while/ })
export const FOR = createToken({ name: "for", pattern: /for/ })
export const IN = createToken({ name: "in", pattern: /in/ })
export const IMPORT = createToken({ name: "import", pattern: /import/ })
export const WITH = createToken({ name: "with", pattern: /with/ })
export const TRUE = createToken({ name: "true", pattern: /true/ })
export const FALSE = createToken({ name: "false", pattern: /false/ })
export const SELF = createToken({ name: "self", pattern: /self/ })
export const AS = createToken({ name: "as", pattern: /as/ })
export const IS = createToken({ name: "is", pattern: /is/ })

export const LIST_START = createToken({ name: "list_start", pattern: /#\[/ })
export const MAP_START = createToken({ name: "map_start", pattern: /#{/ })
export const TUPLE_START = createToken({ name: "tuple_start", pattern: /#\(/ })
export const OBJECT_START = createToken({ name: "object_start", pattern: /@{/ })
export const ARROW = createToken({ name: "arrow", pattern: /->/ })

// literals
export const INT_LITERAL = createToken({ name: "int_literal", pattern: /(\d[\d_]*)/ })
export const FLOAT_LITERAL = createToken({ name: "float_literal", pattern: /(\d[\d_]*\.\d[\d_]*)/ })
export const BOOLEAN_LITERAL = createToken({ name: "boolean_literal", pattern: /(true|false)/ })
export const CHAR_LITERAL = createToken({
	name: "char_literal",
	pattern: /'(\\[ntr'\\]|\\u[\da-fA-F]{4}|[^\\'])'/,
})
export const STRING_LITERAL = createToken({
	name: "string_literal",
	pattern: /"((\\[ntr"\\]|\\u[\da-fA-F]{4}|[^\\"])*)"/,
})

// operators
export const ASSIGN = createToken({ name: "assign", pattern: /=/ })
export const PLUS = createToken({ name: "plus", pattern: /\+/ })
export const PLUS_ASSIGN = createToken({ name: "plus_assign", pattern: /\+=/ })
export const MINUS = createToken({ name: "minus", pattern: /-/ })
export const MINUS_ASSIGN = createToken({ name: "minus_assign", pattern: /-=/ })
export const TIMES = createToken({ name: "times", pattern: /\*/ })
export const TIMES_ASSIGN = createToken({ name: "times_assign", pattern: /\*=/ })
export const DIVIDE = createToken({ name: "divide", pattern: /\// })
export const DIVIDE_ASSIGN = createToken({ name: "divide_assign", pattern: /\/=/ })
export const POWER = createToken({ name: "power", pattern: /\*\*/ })
export const POWER_ASSIGN = createToken({ name: "power_assign", pattern: /\*\*=/ })
export const MODULUS = createToken({ name: "modulus", pattern: /%/ })
export const MODULUS_ASSIGN = createToken({ name: "modulus_assign", pattern: /%=/ })
export const INCREMENT = createToken({ name: "increment", pattern: /\+\+/ })
export const DECREMENT = createToken({ name: "decrement", pattern: /--/ })

export const EQUALS = createToken({ name: "equals", pattern: /==/ })
export const NOT_EQUALS = createToken({ name: "not_equals", pattern: /!=/ })
export const GREATER_THAN = createToken({ name: "greater_than", pattern: />/ })
export const GREATER_THAN_EQUALS = createToken({ name: "greater_than_equals", pattern: />=/ })
export const LESS_THAN = createToken({ name: "less_than", pattern: /</ })
export const LESS_THAN_EQUALS = createToken({ name: "less_than_equals", pattern: /<=/ })

export const AND = createToken({ name: "and", pattern: /&&/ })
export const OR = createToken({ name: "or", pattern: /\|\|/ })
export const NOT = createToken({ name: "not", pattern: /!/ })

export const NOT_IN = createToken({ name: "not_in", pattern: /!in/ })

// types
export const INT_TYPE = createToken({ name: "int", pattern: /int/ })
export const FLOAT_TYPE = createToken({ name: "float", pattern: /float/ })
export const BOOLEAN_TYPE = createToken({ name: "boolean", pattern: /boolean/ })
export const CHAR_TYPE = createToken({ name: "char", pattern: /char/ })
export const STRING_TYPE = createToken({ name: "string", pattern: /string/ })
export const VOID_TYPE = createToken({ name: "void", pattern: /void/ })

export const allTokens = [
	FLOAT_LITERAL,
	INT_LITERAL,
	CHAR_LITERAL,
	STRING_LITERAL,
	LIST_START,
	MAP_START,
	TUPLE_START,
	OBJECT_START,

	INT_TYPE,
	FLOAT_TYPE,
	BOOLEAN_TYPE,
	CHAR_TYPE,
	STRING_TYPE,
	VOID_TYPE,

	LET,
	MUT,
	IF,
	ELSE,
	WHILE,
	FOR,
	IN,
	IMPORT,
	WITH,
	TRUE,
	FALSE,
	SELF,
	AS,
	IS,

	INCREMENT,
	DECREMENT,
	PLUS_ASSIGN,
	PLUS,
	MINUS_ASSIGN,
	MINUS,
	TIMES_ASSIGN,
	TIMES,
	DIVIDE_ASSIGN,
	DIVIDE,
	POWER_ASSIGN,
	POWER,
	MODULUS_ASSIGN,
	MODULUS,
	NOT_EQUALS,
	EQUALS,
	GREATER_THAN_EQUALS,
	GREATER_THAN,
	LESS_THAN_EQUALS,
	LESS_THAN,
	ASSIGN,
	AND,
	OR,
	NOT_IN,
	NOT,

	NEWLINE,
	WHITESPACE,
	DIGIT,
	COMMA,
	PERIOD,
	LEFT_PAREN,
	RIGHT_PAREN,
	LEFT_BRACKET,
	RIGHT_BRACKET,
	LEFT_BRACE,
	RIGHT_BRACE,
	HASH,
	AT_SIGN,
	PIPE,
	QUOTE,
	APOSTROPHE,
	BACKTICK,
	COLON,
	SEMICOLON,
	UNDERSCORE,
	SLASH,
	BACKSLASH,
	TILDE,
	IDENTIFIER,
	ANY,
]
