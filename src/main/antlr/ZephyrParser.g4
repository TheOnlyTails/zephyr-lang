parser grammar ZephyrParser;
@parser::header {
    import com.theonlytails.zephyr_lang.UtilsKt;
    import java.util.stream.Collectors;
}
options { tokenVocab = ZephyrLexer; }

singleExpr : expr EOF;
module : (topLevel ((SEMICOLON | NEWLINE) topLevel)* (SEMICOLON | NEWLINE)?)? EOF;

topLevel : let | external;

expr : operand=expr op=INCREMENT # postfixOp
    | operand=expr op=DECREMENT # postfixOp
    | <assoc=right> lhs=expr op=POWER_OP rhs=expr # binaryOp
    | lhs=expr op=TIMES_OP rhs=expr # binaryOp
    | lhs=expr op=SLASH rhs=expr # binaryOp
    | lhs=expr op=MODULUS_OP rhs=expr # binaryOp
    | lhs=expr op=PLUS_OP rhs=expr # binaryOp
    | lhs=expr op=MINUS_OP rhs=expr # binaryOp
    | op=MINUS_OP operand=expr # prefixOp
    | op=NOT_OP operand=expr # prefixOp
    | lhs=expr op=EQUALS_OP rhs=expr # binaryOp
    | lhs=expr op=NOT_EQUALS_OP rhs=expr # binaryOp
    | lhs=expr op=LESS_THAN_OP rhs=expr # binaryOp
    | lhs=expr op=LESS_THAN_EQUALS_OP rhs=expr # binaryOp
    | lhs=expr op=GREATER_THAN_OP rhs=expr # binaryOp
    | lhs=expr op=GREATER_THAN_EQUALS_OP rhs=expr # binaryOp
    | lhs=expr op=AND_OP rhs=expr # binaryOp
    | lhs=expr op=OR_OP rhs=expr # binaryOp
    | lhs=expr op=IN_OP rhs=expr # binaryOp
    | lhs=expr op=NOT_IN_OP rhs=expr # binaryOp
    | lhs=expr op=RANGE_OP rhs=expr # binaryOp
    | lhs=expr op=RANGE_INC_OP rhs=expr # binaryOp
    | cast # castExpr
    | typeCheck # typeCheckExpr
    | float # floatExpr
    | integer # integerExpr
    | CHAR # charExpr
    | STRING # stringExpr
    | boolean # booleanExpr
    | list # listExpr
    | map # mapExpr
    | tuple # tupleExpr
    | object # objectExpr
    | closure # closureExpr
    | call # callExpr
    | memberAccess # memberAccessExpr
    | indexAccess # indexAccessExpr
    | assignment # assignmentExpr
    | importExpr # importExpression
    | ident # identExpr
    | let # letExpr
    | if # ifExpr
    | while # whileExpr
    | for # forExpr
    | codeBlock # codeBlockExpr
    | parens # parensExpr
    ;

term : ident | float | integer | CHAR | STRING | boolean | list | map | tuple | object | closure | parens;

float : FLOAT exponent=EXPONENT?;
integer : number=intNum exponent=EXPONENT?;
intNum : DECIMAL # decimal
    | BINARY # binary
    | OCTAL # octal
    | HEXADECIMAL # hex
    ;
boolean : TRUE | FALSE;

list : LIST_START (elements+=expr (COMMA elements+=expr)* COMMA?)? RBRACKET;
map : MAP_START (entries+=mapEntry (COMMA entries+=mapEntry)* COMMA?)? RBRACE;
tuple : TUPLE_START (members+=expr (COMMA members+=expr)* COMMA?)? RPAREN;
object : OBJECT_START (fields+=objectField (COMMA fields+=objectField)* COMMA?)? RBRACE;
mapEntry : key=expr COLON value=expr;
objectField : mutable=MUT? name=IDENT (COLON type)? ASSIGN_NONE value=expr;

closure : LPAREN (params+=closureParam (COMMA params+=closureParam)* COMMA?)? RPAREN (COLON returnType=type)? ARROW body=expr;
closureParam : name=IDENT COLON type;

ident : name=IDENT;

call : term LPAREN (args+=expr (COMMA args+=expr)* COMMA?)? RPAREN;

let : LET mutable=MUT? (parent=type DOT)? name=IDENT (COLON typeAnn=type)? ASSIGN_NONE value=expr;
external : EXTERNAL mutable=MUT? (parent=type DOT)? name=IDENT COLON typeAnn=type;

assignment : old=ident op=assignmentOp new=expr # assignIdent
    | old=memberAccess op=assignmentOp new=expr # assignMember
    | old=indexAccess  op=assignmentOp new=expr # assignIndex
    ;
assignmentOp : ASSIGN_NONE # assignNone
    | ASSIGN_PLUS # assignPlus
    | ASSIGN_MINUS # assignMinus
    | ASSIGN_TIMES # assignTimes
    | ASSIGN_DIVIDE # assignDivide
    | ASSIGN_MODULUS # assignModulus
    | ASSIGN_POWER # assignPower
    | ASSIGN_AND # assignAnd
    | ASSIGN_OR # assignOr
    ;

memberAccess : parent=term DOT member=IDENT;
indexAccess : parent=term LBRACKET index=expr RBRACKET;

if : IF LPAREN condition=expr RPAREN then=expr (ELSE otherwise=expr)?;
while : WHILE LPAREN condition=expr RPAREN body=expr;
for : FOR LPAREN name=IDENT IN_OP iterable=expr RPAREN body=expr;

// thanks to @tardiinsanity and @gprinawe on discord for your help on this
importExpr : IMPORT LPAREN path=importSpecifier RPAREN (WITH LPAREN (idents+=importIdent (COMMA idents+=importIdent)* COMMA?)? RPAREN)?;
importSpecifier : IMPORT_STD COLON moduleName=IDENT # importPathStd
    | IMPORT_LIB COLON moduleName=IDENT # importPathLib
    | {UtilsKt.isValidPath($path.stream().map(part -> part.getText()).collect(Collectors.joining()))}? path+=~RPAREN+ # importPathFile
    ;
importIdent : name=IDENT (AS alias=IDENT)?;

cast : value=term AS toType=type;
typeCheck : value=term IS isType=type;

codeBlock : LBRACE (body+=expr ((SEMICOLON | NEWLINE) body+=expr)* (SEMICOLON | NEWLINE)?)? RBRACE;

parens : LPAREN expr RPAREN;

type : PIPE? members+=typeLiteral (PIPE members+=typeLiteral)*;
typeLiteral : INT_TYPE # intType
    | FLOAT_TYPE # floatType
    | CHAR_TYPE # charType
    | STRING_TYPE # stringType
    | BOOLEAN_TYPE # booleanType
    | VOID_TYPE # voidType
    | LIST_START listEl=type RBRACKET # listType
    | MAP_START key=type COLON value=type RBRACE # mapType
    | TUPLE_START (members+=type (COMMA members+=type)* COMMA?)? RPAREN # tupleType
    | OBJECT_START # objectType;
