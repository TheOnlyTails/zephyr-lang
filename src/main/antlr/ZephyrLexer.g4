lexer grammar ZephyrLexer;

// keywords
LET : 'let';
MUT : 'mut';
EXTERNAL : 'external';
IF : 'if';
ELSE : 'else';
WHILE : 'while';
FOR : 'for';
IMPORT : 'import';
WITH : 'with';
AS : 'as';
IS : 'is';
TRUE : 'true';
FALSE : 'false';

// types
INT_TYPE : 'int';
FLOAT_TYPE : 'float';
CHAR_TYPE : 'char';
STRING_TYPE : 'string';
BOOLEAN_TYPE : 'boolean';
VOID_TYPE : 'void';

SEMICOLON : ';';
LPAREN : '(';
RPAREN : ')';
LBRACKET : '[';
RBRACKET : ']';
LBRACE : '{';
RBRACE : '}';
COLON : ':';
COMMA : ',';
DOT : '.';
PIPE : '|';
QUOTE : '"';
SLASH : '/';
APOSTROPHE : '\'';
NEWLINE : '\n';

// collections
LIST_START : '#[';
MAP_START : '#{';
TUPLE_START : '#(';
OBJECT_START : '@{';

// closures
ARROW : '->';

IDENT : Letter AlphaNumLetter*;

// operators
ASSIGN_NONE : '=';
ASSIGN_PLUS : '+=';
ASSIGN_MINUS : '-=';
ASSIGN_TIMES : '*=';
ASSIGN_DIVIDE : '/=';
ASSIGN_MODULUS : '%=';
ASSIGN_POWER : '**=';
ASSIGN_AND : '&&=';
ASSIGN_OR : '||=';

PLUS_OP : '+';
MINUS_OP : '-';
TIMES_OP : '*';
MODULUS_OP : '%';
POWER_OP : '**';
EQUALS_OP : '==';
NOT_EQUALS_OP : '!=';
LESS_THAN_OP : '<';
LESS_THAN_EQUALS_OP : '<=';
GREATER_THAN_OP : '>';
GREATER_THAN_EQUALS_OP : '>=';
AND_OP : '&&';
OR_OP : '||';
IN_OP : 'in';
NOT_IN_OP : '!in';
RANGE_OP : '..';
RANGE_INC_OP : '..=';

NOT_OP : '!';

INCREMENT : '++';
DECREMENT : '--';

WHITESPACE : [ \t\r\n\u000C]+ -> channel(HIDDEN);
COMMENT_LINE : '//' ~[\r\n]* -> channel(HIDDEN);
COMMENT_BLOCK : '/*' .*? '*/' -> channel(HIDDEN);

// numbers
fragment DECIMAL_INT : Digit ((Digit | '_')* Digit)?;
FLOAT : DECIMAL_INT '.' DECIMAL_INT;
DECIMAL : DECIMAL_INT;
BINARY : '0b' [01_]* [01] [01_]*;
OCTAL : '0o' (OctDigit | '_')* OctDigit (OctDigit | '_')*;
HEXADECIMAL : '0x' (HexDigit | '_')* HexDigit (HexDigit | '_')*;
EXPONENT : 'e' DECIMAL_INT;

TEXT_LITERAL_CHAR : NormalChar | EscapeChar | UnicodeChar;
CHAR : APOSTROPHE TEXT_LITERAL_CHAR APOSTROPHE;
STRING : QUOTE TEXT_LITERAL_CHAR* QUOTE;

// import module specifier
IMPORT_STD : 'std';
IMPORT_LIB : 'lib';

fragment NormalChar : ~('"' | '\'' | '\\');
fragment EscapeChar : '\\' ["'\\bfnrt];
fragment UnicodeChar : '\\u' HexDigit HexDigit HexDigit HexDigit HexDigit? HexDigit?;
fragment AlphaNumLetter : Letter | Digit;
fragment Digit : [0-9];
fragment HexDigit : [0-9a-fA-F];
fragment OctDigit : [0-7];
fragment Letter
    : [a-zA-Z$_] // these are the "java letters" below 0x7F
    | ~[\u0000-\u007F\uD800-\uDBFF] // covers all characters above 0x7F which are not a surrogate
    | [\uD800-\uDBFF] [\uDC00-\uDFFF] // covers UTF-16 surrogate pairs encodings for U+10000 to U+10FFFF
    ;
