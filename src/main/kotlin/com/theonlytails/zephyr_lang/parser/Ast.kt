package com.theonlytails.zephyr_lang.parser

import com.theonlytails.zephyr_lang.ZephyrError
import com.theonlytails.zephyr_lang.err
import com.theonlytails.zephyr_lang.ok
import com.theonlytails.zephyr_lang.parser.Operator.*
import com.theonlytails.zephyr_lang.parser.ZephyrType.FloatType
import com.theonlytails.zephyr_lang.stdlib.StdlibModule
import okio.Path

data class ZephyrModule(val members: Map<EnvId, Declaration>)

sealed interface Declaration {
    val type: ZephyrType
    val mutable: Boolean

    data class Actual(
        override val type: ZephyrType,
        val expr: ZephyrExpression,
        override val mutable: Boolean = false
    ) : Declaration

    data class Unknown(
        override val type: ZephyrType,
        override val mutable: Boolean = false
    ) : Declaration
}

sealed interface ZephyrExpression {
    sealed interface ZephyrLiteralExpr<T> : ZephyrExpression {
        val value: T
    }

    @JvmInline
    value class IntExpr(override val value: Int) : ZephyrLiteralExpr<Int>

    @JvmInline
    value class FloatExpr(override val value: Double) : ZephyrLiteralExpr<Double>

    @JvmInline
    value class CharExpr(override val value: Char) : ZephyrLiteralExpr<Char>

    @JvmInline
    value class StringExpr(override val value: String) : ZephyrLiteralExpr<String>

    @JvmInline
    value class BooleanExpr(override val value: Boolean) : ZephyrLiteralExpr<Boolean>

    @JvmInline
    value class ListExpr(override val value: List<ZephyrExpression>) :
        ZephyrLiteralExpr<List<ZephyrExpression>>

    @JvmInline
    value class MapExpr(override val value: Map<ZephyrExpression, ZephyrExpression>) :
        ZephyrLiteralExpr<Map<ZephyrExpression, ZephyrExpression>>

    @JvmInline
    value class TupleExpr(override val value: List<ZephyrExpression>) :
        ZephyrLiteralExpr<List<ZephyrExpression>>

    @JvmInline
    value class ObjectExpr(override val value: Map<Name, Declaration.Actual>) :
        ZephyrLiteralExpr<Map<Name, Declaration.Actual>>

    data class ClosureExpr(
        val params: Map<Name, ZephyrType>,
        val returnType: ZephyrType?,
        val body: ZephyrExpression
    ) : ZephyrExpression

    @JvmInline
    value class IdentExpr(val name: EnvId.Global) : ZephyrExpression

    data class CallExpr(val func: ZephyrExpression, val args: List<ZephyrExpression>) : ZephyrExpression

    data class MemberAccessExpr(val parent: ZephyrExpression, val member: EnvId.Member) : ZephyrExpression

    data class IndexAccessExpr(val parent: ZephyrExpression, val index: ZephyrExpression) : ZephyrExpression

    data class AssignmentExpr(val target: AssignTarget, val op: AssignmentOp, val new: ZephyrExpression) :
        ZephyrExpression {
        sealed interface AssignTarget {
            @JvmInline
            value class Variable(val ident: IdentExpr) : AssignTarget {
                override fun toEnvId() = ident.name
            }

            @JvmInline
            value class MemberAccess(val expr: MemberAccessExpr) : AssignTarget {
                override fun toEnvId() = expr.member
            }

            fun toEnvId(): EnvId
            fun type(env: ParserEnv) = type(env.toTypeMap())
            fun type(env: Map<EnvId, ZephyrType>): Result<ZephyrType> = when (this) {
                is MemberAccess -> expr.type(env)
                is Variable -> ident.type(env)
            }
        }
    }

    data class BinaryOpExpr(val lhs: ZephyrExpression, val op: BinaryOp, val rhs: ZephyrExpression) :
        ZephyrExpression

    data class PrefixOpExpr(val op: PrefixOp, val operand: ZephyrExpression) : ZephyrExpression

    data class PostfixOpExpr(val operand: ZephyrExpression, val op: PostfixOp) : ZephyrExpression

    data class LetExpr(val name: EnvId, val data: Declaration.Actual) : ZephyrExpression

    data class ExternalExpr(val name: EnvId, val data: Declaration.Unknown) : ZephyrExpression

    data class IfExpr(val condition: ZephyrExpression, val then: ZephyrExpression, val otherwise: ZephyrExpression?) :
        ZephyrExpression

    data class WhileExpr(val condition: ZephyrExpression, val body: ZephyrExpression) : ZephyrExpression

    data class ForExpr(val name: Name, val iterable: ZephyrExpression, val body: ZephyrExpression) : ZephyrExpression

    data class ImportExpr(val path: ImportPath, val idents: Map<Name, Alias?>?) : ZephyrExpression

    data class CastExpr(val expr: ZephyrExpression, val type: ZephyrType) : ZephyrExpression

    data class TypeCheckExpr(val expr: ZephyrExpression, val type: ZephyrType) : ZephyrExpression

    data class CodeBlockExpr(val body: List<ZephyrExpression>) : ZephyrExpression
}

sealed interface ImportPath {
    override fun toString(): String

    data class File(val path: Path) : ImportPath {
        override fun toString() = "$path"
    }

    data class Stdlib(val module: StdlibModule) : ImportPath {
        override fun toString() = "std:${module.moduleName}"
    }

//    data class
}

sealed interface ZephyrType {
    sealed interface ZephyrIterableType<Element : ZephyrType, Index : ZephyrType> : ZephyrType {
        val itemType: Element
        val indexType: Index
    }

    sealed interface ZephyrOrderedType : ZephyrType

    data object IntType : ZephyrOrderedType {
        override fun toString() = "int"
    }

    data object FloatType : ZephyrType {
        override fun toString() = "float"
    }

    data object CharType : ZephyrOrderedType {
        override fun toString() = "char"
    }

    data object StringType : ZephyrIterableType<CharType, IntType> {
        override val itemType = CharType
        override val indexType = IntType

        override fun toString() = "string"
    }

    data object BooleanType : ZephyrType {
        override fun toString() = "boolean"
    }

    data object VoidType : ZephyrType {
        override fun toString() = "void"
    }

    data class ListType<T : ZephyrType>(override val itemType: T) : ZephyrIterableType<T, IntType> {
        override fun toString() = "#[$itemType]"
        override val indexType = IntType
    }

    data class MapType<K : ZephyrType, V : ZephyrType>(val keyType: K, val valueType: V) : ZephyrIterableType<V, K> {
        override val itemType = valueType
        override val indexType = keyType
        override fun toString() = "#{$keyType: $valueType}"
    }

    data class TupleType(val itemTypes: List<ZephyrType>) : ZephyrIterableType<UnionType, IntType> {
        override val itemType = UnionType(itemTypes)
        override val indexType = IntType

        override fun toString() = "#(${itemTypes.joinToString()})"

        constructor(vararg members: ZephyrType) : this(members.toList())
    }

    data class ObjectType(val fields: Map<String, ZephyrType>) : ZephyrIterableType<TupleType, StringType> {
        override val itemType = TupleType(StringType, UnionType(fields.values.toList()))
        override val indexType = StringType

        constructor(vararg itemTypes: Pair<String, ZephyrType>) : this(mapOf(*itemTypes))

        override fun toString() = "@{${fields.entries.joinToString { (k, v) -> "$k: $v" }}"
    }

    data class ClosureType(val args: List<ZephyrType>, val returnType: ZephyrType) : ZephyrType {
        override fun toString() = "(${args.joinToString()}) -> $returnType"
    }

    data class UnionType(val members: List<ZephyrType>) : ZephyrType {
        override fun toString() = members.joinToString(" | ")

        constructor(vararg members: ZephyrType) : this(members.toList())
    }

    override fun toString(): String
}

sealed interface Operator {
    val operator: String
    val humanName: String

    enum class AssignmentOp(
        override val operator: String,
        override val humanName: String,
        val resultingType: (ZephyrType, ZephyrType) -> Result<ZephyrType>,
    ) : Operator {
        None(
            "=",
            "regular",
            { lhs, rhs ->
                if (rhs assignableTo lhs) lhs.ok()
                else ZephyrError.InvalidOperation(None, lhs, rhs).err()
            }
        ),
        Plus(
            "+=",
            "addition",
            { lhs, rhs ->
                if (lhs assignableTo numberType && rhs assignableTo lhs) lhs.ok()
                else ZephyrError.InvalidOperation(Plus, lhs, rhs).err()
            }
        ),
        Minus("-=", "subtraction"),
        Times("*=", "multiplication"),
        Divide("/=", "division", { FloatType }),
        Modulus("%=", "remainder"),
        Power("**=", "exponentiation", { FloatType }),
        And("&&=", "and-ing"),
        Or("||=", "or-ing");

    }

    enum class BinaryOp(override val operator: String, override val humanName: String) : Operator {
        Plus("+", "addition"),
        Minus("-", "subtraction"),
        Times("*", "multiplication"),
        Divide("/", "division"),
        Modulus("%", "remainder"),
        Power("**", "exponentiation"),
        Equals("==", "equality"),
        NotEquals("!=", "non-equality"),
        LessThan("<", "less-than"),
        LessThanEquals("<=", "less-than-equals"),
        GreaterThan(">", "greater-than"),
        GreaterThanEquals(">=", "greater-than-equals"),
        And("&&", "and-ing"),
        Or("||", "or-ing"),
        In("in", "inclusion"),
        NotIn("!in", "non-inclusion"),
        Range("..", "exclusive range"),
        RangeInclusive("..=", "inclusive range");

        override fun toString() = operator

        companion object {
            val numberOps = arrayOf(Plus, Minus, Times, Divide, Modulus, Power)
            val equalityOps = arrayOf(Equals, NotEquals)
            val comparisonOps = arrayOf(LessThan, LessThanEquals, GreaterThan, GreaterThanEquals)
            val booleanOps = arrayOf(And, Or)
            val rangeOps = arrayOf(Range, RangeInclusive)

            fun fromString(op: String): Result<BinaryOp> {
                return Result.success(
                    when (op) {
                        "+" -> Plus
                        "-" -> Minus
                        "*" -> Times
                        "/" -> Divide
                        "%" -> Modulus
                        "**" -> Power
                        "==" -> Equals
                        "!=" -> NotEquals
                        "<" -> LessThan
                        "<=" -> LessThanEquals
                        ">" -> GreaterThan
                        ">=" -> GreaterThanEquals
                        "&&" -> And
                        "||" -> Or
                        "in" -> In
                        "!in" -> NotIn
                        ".." -> Range
                        "..=" -> RangeInclusive
                        else -> return Result.failure(IllegalArgumentException("$op is not a valid binary operator"))
                    }
                )
            }
        }
    }

    enum class PrefixOp(override val operator: String, override val humanName: String) : Operator {
        Not("!", "not"),
        Negate("-", "negation");

        override fun toString() = operator

        companion object {
            fun fromString(op: String): Result<PrefixOp> {
                return Result.success(
                    when (op) {
                        "!" -> Not
                        "-" -> Negate
                        else -> return Result.failure(IllegalArgumentException("$op is not a valid prefix operator."))
                    }
                )
            }
        }
    }

    enum class PostfixOp(override val operator: String, override val humanName: String) : Operator {
        Increment("++", "increment"),
        Decrement("--", "decrement");

        override fun toString() = operator

        companion object {
            fun fromString(op: String): Result<PostfixOp> {
                return Result.success(
                    when (op) {
                        "++" -> Increment
                        "--" -> Decrement
                        else -> return Result.failure(IllegalArgumentException("$op is not a valid postfix operator."))
                    }
                )
            }
        }
    }
}

typealias Name = String
typealias Alias = String
