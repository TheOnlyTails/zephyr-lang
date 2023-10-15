package com.theonlytails.zephyr_lang.parser

import ZephyrLexer
import ZephyrParser
import ZephyrParser.*
import com.github.ajalt.mordant.rendering.TextColors.red
import com.theonlytails.zephyr_lang.MultiException
import com.theonlytails.zephyr_lang.ZephyrError
import com.theonlytails.zephyr_lang.err
import com.theonlytails.zephyr_lang.ok
import com.theonlytails.zephyr_lang.parser.Operator.AssignmentOp
import com.theonlytails.zephyr_lang.parser.Operator.BinaryOp
import com.theonlytails.zephyr_lang.parser.ZephyrExpression.AssignmentExpr.AssignTarget
import com.theonlytails.zephyr_lang.parser.ZephyrType.*
import okio.Path
import org.antlr.runtime.NoViableAltException
import org.antlr.v4.runtime.*
import org.antlr.v4.runtime.Parser
import org.antlr.v4.runtime.atn.ATNConfigSet
import org.antlr.v4.runtime.dfa.DFA
import java.io.FileNotFoundException
import java.io.InputStream
import java.util.*
import kotlin.io.path.inputStream
import kotlin.math.pow

enum class ParseTarget {
    Module, Expr
}

sealed interface AntlrParseResult {
    @JvmInline
    value class Module(val module: ModuleContext) : AntlrParseResult {
        override fun isSuccess() = true
    }

    @JvmInline
    value class Expr(val singleExpr: SingleExprContext) : AntlrParseResult {
        override fun isSuccess() = true
    }

    @JvmInline
    value class Failure(val errors: List<Error>) : AntlrParseResult {
        override fun isSuccess() = false
    }

    fun isSuccess(): Boolean
}

object Parser {
    fun parse(input: String, target: ParseTarget = ParseTarget.Module, initialEnv: ParserEnv = mutableMapOf()) =
        parse(input.byteInputStream(), target, initialEnv)

    fun parse(input: Path, target: ParseTarget = ParseTarget.Module, initialEnv: ParserEnv = mutableMapOf()) = try {
        parse(input.toNioPath().inputStream(), target, initialEnv)
    } catch (e: FileNotFoundException) {
        ZephyrError.FileNotFound(input).err()
    }

    private fun parse(input: InputStream, target: ParseTarget, initialEnv: ParserEnv): Result<ZephyrModule> {
        return when (val res = antlrParse(input, target)) {
            is AntlrParseResult.Failure -> Result.failure(
                Error(res.errors.joinToString("\n") { red(it.localizedMessage) })
            )

            is AntlrParseResult.Module -> {
                val context = res.module
                val env: ParserEnv = initialEnv
                val declarations = context.topLevel().map {
                    if (it.let() != null) parseExpr(it.let(), env).getOrElse { e -> return e.err() }.run {
                        name to data
                    }
                    else parseExpr(it.external(), env).getOrElse { e -> return e.err() }.run {
                        name to data
                    }
                }.toMap()

                ZephyrModule(declarations).ok()
            }

            is AntlrParseResult.Expr -> {
                val context = res.singleExpr
                val expr = parseExpr(context.expr(), mutableMapOf()).getOrElse { return it.err() }
                val type = expr.type(emptyMap()).getOrElse { return it.err() }

                Result.success(
                    ZephyrModule(mapOf(EnvId.Global("_value") to Declaration.Actual(type, expr)))
                )
            }
        }
    }

    private fun parseExpr(ctx: ExprContext, env: ParserEnv) = when (ctx) {
        is IntegerExprContext -> parseExpr(ctx.integer())
        is FloatExprContext -> parseExpr(ctx.float_())
        is CharExprContext -> parseExpr(ctx)
        is StringExprContext -> parseExpr(ctx)
        is BooleanExprContext -> parseExpr(ctx.boolean_())
        is ListExprContext -> parseExpr(ctx.list(), env)
        is MapExprContext -> parseExpr(ctx.map(), env)
        is TupleExprContext -> parseExpr(ctx.tuple(), env)
        is ObjectExprContext -> parseExpr(ctx.`object`(), env)
        is ClosureExprContext -> parseExpr(ctx.closure(), env)
        is IdentExprContext -> parseExpr(ctx.ident(), env)
        is CallExprContext -> parseExpr(ctx.call(), env)
        is IndexAccessExprContext -> parseExpr(ctx.indexAccess(), env)
        is MemberAccessExprContext -> parseExpr(ctx.memberAccess(), env)
        is LetExprContext -> parseExpr(ctx.let(), env)
        is AssignmentExprContext -> parseExpr(ctx.assignment(), env)
        is IfExprContext -> parseExpr(ctx.if_(), env)
        is WhileExprContext -> parseExpr(ctx.while_(), env)
        is ForExprContext -> parseExpr(ctx.for_(), env)
        is ImportExpressionContext -> parseExpr(ctx.importExpr(), env)
        is CastExprContext -> parseExpr(ctx.cast(), env)
        is TypeCheckExprContext -> parseExpr(ctx.typeCheck(), env)
        is CodeBlockExprContext -> parseExpr(ctx.codeBlock(), env)
        is ParensExprContext -> parseExpr(ctx.parens(), env)
        is BinaryOpContext -> parseExpr(ctx, env)
        else -> NoViableAltException().err()
    }

    private fun parseTerm(ctx: TermContext, env: ParserEnv) =
        parseExpr(if (ctx.parens() != null) ctx.parens().expr() else ctx.getChild(ExprContext::class.java, 0), env)

    private fun parseExpr(ctx: IntegerContext): Result<ZephyrExpression.IntExpr> {
        val value = when (val number = ctx.number) {
            is DecimalContext -> number.text.replace("_", "").toInt()
            is BinaryContext -> number.text.removePrefix("0b").replace("_", "").toInt(2)
            is OctalContext -> number.text.removePrefix("0o").replace("_", "").toInt(8)
            is HexContext -> number.text.removePrefix("0x").replace("_", "").toInt(16)
            else -> return NoViableAltException().err()
        }
        val exponent = 10.0.pow(if (ctx.exponent != null) ctx.exponent.text.toInt() else 0).toInt()

        return ZephyrExpression.IntExpr(value * exponent).ok()
    }


    private fun parseExpr(ctx: FloatContext): Result<ZephyrExpression.FloatExpr> {
        val value = ctx.FLOAT().text.replace("_", "").toDouble()
        // this will cancel down to 1 if there's no exponent
        val exponent = 10.0.pow(if (ctx.exponent != null) ctx.exponent.text.toInt() else 0).toInt()

        return ZephyrExpression.FloatExpr(value * exponent).ok()
    }

    private fun parseExpr(ctx: BooleanContext) =
        ZephyrExpression.BooleanExpr(ctx.FALSE() == null).ok()

    private val escapeCharRegex = """\\['"\\bfnrt]""".toRegex()
    private val unicodeCharRegex = """\\u[\dA-Fa-f]{4,6}""".toRegex()
    private fun parseExpr(ctx: CharExprContext) =
        ZephyrExpression.CharExpr(parseEscapeCharacter(ctx.CHAR().text.removeSurrounding("'"))).ok()

    private fun parseExpr(ctx: StringExprContext) =
        ZephyrExpression.StringExpr(
            """"($unicodeCharRegex|$escapeCharRegex|[^"'\\])*""""
                .toRegex()
                .findAll(ctx.STRING().text)
                .joinToString("") { "${parseEscapeCharacter(it.value)}" }
        ).ok()

    private fun parseEscapeCharacter(char: String) = if (char.matches(escapeCharRegex)) when (char[1]) {
        'b' -> '\b'
        'n' -> '\n'
        'f' -> '\u000c'
        'r' -> '\r'
        't' -> '\t'
        else -> char[1]
    }
    else if (char.matches(unicodeCharRegex)) char.removePrefix("\\u").toInt(16).toChar()
    else char[0]

    private fun parseExpr(list: ListContext, env: ParserEnv): Result<ZephyrExpression.ListExpr> =
        ZephyrExpression.ListExpr(list.elements.map {
            parseExpr(it, env).getOrElse { e -> return e.err() }
        }).ok()

    private fun parseExpr(map: MapContext, env: ParserEnv): Result<ZephyrExpression.MapExpr> =
        ZephyrExpression.MapExpr(map.entries.map {
            parseExpr(it.key, env).getOrElse { e -> return e.err() } to
                    parseExpr(it.value, env).getOrElse { e -> return e.err() }
        }.toMap()).ok()

    private fun parseExpr(tuple: TupleContext, env: ParserEnv): Result<ZephyrExpression.TupleExpr> =
        ZephyrExpression.TupleExpr(tuple.members.map {
            parseExpr(it, env).getOrElse { e -> return e.err() }
        }).ok()

    private fun parseExpr(obj: ObjectContext, env: ParserEnv): Result<ZephyrExpression.ObjectExpr> =
        ZephyrExpression.ObjectExpr(obj.fields.map { field ->
            val fieldValue = parseExpr(field.value, env).getOrElse { return it.err() }

            field.name.text to Declaration.Actual(
                (field.type()?.let { parseType(it) } ?: fieldValue.type(env)).getOrElse { return it.err() },
                fieldValue,
                field.mutable != null
            )
        }.toMap()).ok()

    private fun parseExpr(closure: ClosureContext, env: ParserEnv): Result<ZephyrExpression.ClosureExpr> {
        val params = closure.params
            .map { it.IDENT().text to parseType(it.type()).getOrElse { e -> return e.err() } }
            .toMap()

        return ZephyrExpression.ClosureExpr(
            params,
            parseType(closure.returnType).getOrElse { return it.err() },
            parseExpr(
                closure.body,
                (env + params.map { (name, type) -> EnvId.Global(name) to Declaration.Unknown(type) }).toMutableMap()
            ).getOrElse { return it.err() }
        ).ok()
    }

    private fun parseExpr(ident: IdentContext, env: ParserEnv): Result<ZephyrExpression.IdentExpr> {
        if (EnvId.Global(ident.name.text) !in env) return ZephyrError.IdentNotFound(
            EnvId.Global(ident.name.text),
            env.keys
        ).err()
        return ZephyrExpression.IdentExpr(EnvId.Global(ident.name.text)).ok()
    }

    private fun parseExpr(call: CallContext, env: ParserEnv): Result<ZephyrExpression.CallExpr> {
        val func = parseExpr(call.expr, env).getOrElse { return it.err() }
        val funcType = func.type(env).getOrElse { return it.err() }

        if (funcType !is ClosureType) return ZephyrError.NonClosureCall(funcType).err()

        val params = call.args.map { parseExpr(it, env).getOrElse { e -> return e.err() } }
        val paramTypes = params.map { it.type(env).getOrElse { e -> return e.err() } }
        val errors = paramTypes.zip(funcType.args).mapNotNull { (param, arg) ->
            if (!(param assignableTo arg)) ZephyrError.WrongParamType(param, arg) else null
        }

        return if (errors.isNotEmpty()) MultiException(errors).err()
        else ZephyrExpression.CallExpr(func, params).ok()
    }

    /**
     * Parses an index access expression from the provided context.
     *
     * @param ctx The context representing the index access expression.
     * @param env The parser environment.
     * @return A Result containing the parsed index access expression if successful, or an error if parsing fails.
     */
    private fun parseExpr(ctx: IndexAccessContext, env: ParserEnv): Result<ZephyrExpression.IndexAccessExpr> {
        val parent = parseTerm(ctx.parent, env).getOrElse { return it.err() }
        val parentType = parent.type(env).getOrElse { return it.err() }
        if (parentType !is ZephyrIterableType<*, *>) return ZephyrError.UnindexableType(parentType).err()

        val index = parseExpr(ctx.index, env).getOrElse { return it.err() }
        val indexType = index.type(env).getOrElse { return it.err() }
        if (!(indexType assignableTo parentType.itemType))
            return ZephyrError.InvalidIndexType(parentType, indexType).err()

        // For objects and tuples, which have a known set of contained values at compile-time,
        // we add a check for the validity of literal index expressions
        // so `#(1, 2, 3)[4]` would get caught at compile-time
        return when (parentType) {
            is TupleType ->
                if (index is ZephyrExpression.IntExpr && index.value !in parentType.itemTypes.indices)
                    ZephyrError.InvalidIndexValue(index, parentType, parentType.itemTypes.indices).err()
                else ZephyrExpression.IndexAccessExpr(parent, index).ok()

            is ObjectType ->
                if (index is ZephyrExpression.StringExpr && index.value !in parentType.fields.keys)
                    ZephyrError.InvalidIndexValue(index, parentType, parentType.fields.keys).err()
                else ZephyrExpression.IndexAccessExpr(parent, index).ok()

            else -> ZephyrExpression.IndexAccessExpr(parent, index).ok()
        }
    }

    private fun parseExpr(ctx: LetContext, env: ParserEnv): Result<ZephyrExpression.LetExpr> {
        val mutable = ctx.MUT() != null
        val name = ctx.IDENT().text
        val parent = if (ctx.parent != null) parseType(ctx.parent).getOrElse { return it.err() } else null
        val envId = parent?.let { EnvId.Member(name, parent) } ?: EnvId.Global(name)

        if (envId in env) return ZephyrError.DuplicateIdent(name).err()

        val initialValue = parseExpr(ctx.expr(), env).getOrElse { return it.err() }
        val initialValueType = initialValue.type(env).getOrElse { return it.err() }
        val inferredType = ctx.typeAnn?.let { parseType(it) }?.getOrElse { return it.err() } ?: initialValueType

        if (!(initialValueType assignableTo inferredType)) return ZephyrError.UnexpectedType(
            initialValueType,
            inferredType
        ).err()

        val declarationData = Declaration.Actual(inferredType, initialValue, mutable)
        env[envId] = declarationData
        return ZephyrExpression.LetExpr(envId, declarationData).ok()
    }

    private fun parseExpr(ctx: ExternalContext, env: ParserEnv): Result<ZephyrExpression.ExternalExpr> {
        val name = ctx.IDENT().text
        val parent = if (ctx.parent != null) parseType(ctx.parent).getOrElse { return it.err() } else null
        val envId = parent?.let { EnvId.Member(name, parent) } ?: EnvId.Global(name)

        if (envId in env) return ZephyrError.DuplicateIdent(name).err()

        val type = parseType(ctx.typeAnn).getOrElse { return it.err() }
        val mutable = ctx.MUT() != null

        val declarationData = Declaration.Unknown(type, mutable)
        env[envId] = declarationData
        return ZephyrExpression.ExternalExpr(envId, declarationData).ok()
    }

    private fun parseExpr(ctx: MemberAccessContext, env: ParserEnv): Result<ZephyrExpression.MemberAccessExpr> {
        val parent = parseTerm(ctx.parent, env).getOrElse { return it.err() }
        val parentType = parent.type(env).getOrElse { return it.err() }
        val member = ctx.member.text
        val envId = EnvId.Member(member, parentType)
        if (envId !in env) return ZephyrError.IdentNotFound(envId, env.keys).err()

        return ZephyrExpression.MemberAccessExpr(parent, envId).ok()
    }

    private fun parseExpr(
        ctx: BinaryOpContext,
        env: ParserEnv
    ): Result<ZephyrExpression.BinaryOpExpr> {
        val lhs = parseExpr(ctx.lhs, env).getOrElse { return it.err() }
        val lhsType = lhs.type(env).getOrElse { return it.err() }
        val rhs = parseExpr(ctx.rhs, env).getOrElse { return it.err() }
        val rhsType = rhs.type(env).getOrElse { return it.err() }
        val op = BinaryOp.fromString(ctx.op.text).getOrElse { return it.err() }

        if (op == BinaryOp.Plus && lhsType assignableTo StringType && !(rhsType assignableTo StringType))
            return ZephyrError.InvalidOperation(op, lhsType, rhsType).err()

        if (op in BinaryOp.numberOps) {
            if (lhsType is FloatType && !(rhsType assignableTo FloatType))
                return ZephyrError.InvalidOperation(op, lhsType, rhsType).err()

            if (lhsType is IntType && !(rhsType assignableTo IntType))
                return ZephyrError.InvalidOperation(op, lhsType, rhsType).err()
        }

        if (op in BinaryOp.equalityOps && rhsType assignableTo lhsType)
            return ZephyrError.InvalidOperation(op, lhsType, rhsType).err()

        if (op in BinaryOp.comparisonOps && !(lhsType is ZephyrOrderedType && rhsType is ZephyrOrderedType))
            return ZephyrError.InvalidOperation(op, lhsType, rhsType).err()

        if (op in BinaryOp.booleanOps && !(lhsType assignableTo BooleanType && rhsType assignableTo BooleanType))
            return ZephyrError.InvalidOperation(op, lhsType, rhsType).err()

        if (op in BinaryOp.rangeOps && !(lhsType is ZephyrOrderedType && rhsType is ZephyrOrderedType))
            return ZephyrError.InvalidOperation(op, lhsType, rhsType).err()

        return ZephyrExpression.BinaryOpExpr(lhs, op, rhs).ok()
    }

    private fun parseExpr(ctx: AssignmentContext, env: ParserEnv): Result<ZephyrExpression.AssignmentExpr> {
        val target = when (ctx) {
            is AssignIdentContext -> AssignTarget.Variable(parseExpr(ctx.old, env).getOrElse { return it.err() })
            is AssignMemberContext -> AssignTarget.MemberAccess(parseExpr(ctx.old, env).getOrElse { return it.err() })
            else -> return NoViableAltException().err()
        }

        val envId = target.toEnvId()
        val declaration = env[envId] ?: return ZephyrError.IdentNotFound(envId, env.keys).err()
        if (!declaration.mutable) return ZephyrError.AssignmentToImmutable(envId).err()

        val targetType = target.type(env).getOrElse { return it.err() }

        val operatorRule = when (ctx) {
            is AssignIdentContext -> ctx.op
            is AssignMemberContext -> ctx.op
            else -> return NoViableAltException().err()
        }

        val operator = when (operatorRule) {
            is AssignNoneContext -> AssignmentOp.None
            is AssignPlusContext -> AssignmentOp.Plus
            is AssignMinusContext -> AssignmentOp.Minus
            is AssignTimesContext -> AssignmentOp.Times
            is AssignDivideContext -> AssignmentOp.Divide
            is AssignModulusContext -> AssignmentOp.Modulus
            is AssignPowerContext -> AssignmentOp.Power
            is AssignAndContext -> AssignmentOp.And
            is AssignOrContext -> AssignmentOp.Or
            else -> return NoViableAltException().err()
        }

        val new = when (ctx) {
            is AssignIdentContext -> parseExpr(ctx.new_, env)
            is AssignMemberContext -> parseExpr(ctx.new_, env)
            else -> return NoViableAltException().err()
        }.getOrElse { return it.err() }
        val newType = new.type(env).getOrElse { return it.err() }

        if (!(newType assignableTo targetType))
            return ZephyrError.UnexpectedType(newType, targetType).err()

        return ZephyrExpression.AssignmentExpr(target, operator, new).ok()
    }

    internal fun parseType(ctx: TypeContext): Result<ZephyrType> {
        return if (ctx.members.size == 1) parseType(ctx.members[0])
        else Result.success(UnionType(
            ctx.members.map { parseType(it).getOrElse { e -> return e.err() } }
        ))
    }

    private fun parseType(ctx: TypeLiteralContext): Result<ZephyrType> {
        return Result.success(
            when (ctx) {
                is IntTypeContext -> IntType
                is FloatTypeContext -> FloatType
                is CharTypeContext -> CharType
                is StringTypeContext -> StringType
                is BooleanTypeContext -> BooleanType
                is VoidTypeContext -> VoidType
                is ListTypeContext -> ListType(parseType(ctx.listEl).getOrElse { return it.err() })
                is MapTypeContext -> MapType(
                    parseType(ctx.key).getOrElse { return it.err() },
                    parseType(ctx.value).getOrElse { return it.err() }
                )

                is TupleTypeContext -> TupleType(ctx.members.map {
                    parseType(it).getOrElse { e -> return e.err() }
                })

                is ObjectTypeContext -> ObjectType()

                else -> return NoViableAltException().err()
            }
        )
    }

    private fun antlrParse(input: InputStream, target: ParseTarget): AntlrParseResult {
        val errors = mutableListOf<Error>()
        val errorListener = object : ANTLRErrorListener {
            override fun syntaxError(
                recognizer: Recognizer<*, *>?,
                offendingSymbol: Any?,
                line: Int,
                charPositionInLine: Int,
                msg: String?,
                e: RecognitionException?
            ) {
                errors += Error(msg, e)
            }

            override fun reportAmbiguity(
                recognizer: Parser?,
                dfa: DFA?,
                startIndex: Int,
                stopIndex: Int,
                exact: Boolean,
                ambigAlts: BitSet?,
                configs: ATNConfigSet?
            ) {
                // ignored
            }

            override fun reportAttemptingFullContext(
                recognizer: Parser?,
                dfa: DFA?,
                startIndex: Int,
                stopIndex: Int,
                conflictingAlts: BitSet?,
                configs: ATNConfigSet?
            ) {
                // ignored
            }

            override fun reportContextSensitivity(
                recognizer: Parser?,
                dfa: DFA?,
                startIndex: Int,
                stopIndex: Int,
                prediction: Int,
                configs: ATNConfigSet?
            ) {
                // ignored
            }

        }

        val lexer = ZephyrLexer(CharStreams.fromStream(input)).apply {
            removeErrorListeners()
            addErrorListener(errorListener)
        }
        val parser = ZephyrParser(CommonTokenStream(lexer)).apply {
            removeErrorListeners()
            addErrorListener(errorListener)
        }

        return if (errors.isEmpty()) when (target) {
            ParseTarget.Module -> AntlrParseResult.Module(parser.module())
            ParseTarget.Expr -> AntlrParseResult.Expr(parser.singleExpr())
        }
        else AntlrParseResult.Failure(errors)
    }
}
