package com.theonlytails.zephyr_lang.parser

import com.theonlytails.zephyr_lang.ZephyrError
import com.theonlytails.zephyr_lang.err
import com.theonlytails.zephyr_lang.findParentWithFile
import com.theonlytails.zephyr_lang.ok
import com.theonlytails.zephyr_lang.stdlib.StdlibModule
import java.io.File
import java.nio.file.FileSystem
import java.nio.file.Files
import java.nio.file.Path
import kotlin.io.path.div
import kotlin.io.path.extension
import kotlin.io.path.pathString

val numberType = ZephyrType.UnionType(ZephyrType.IntType, ZephyrType.FloatType)

fun ZephyrExpression.type(env: ParserEnv) = type(env.toTypeMap())
fun ZephyrExpression.type(env: Map<EnvId, ZephyrType>): Result<ZephyrType> {
    return when (this) {
        is ZephyrExpression.AssignmentExpr -> {
            val originalType = target.type(env).getOrElse { return it.err() }

            when (op) {
                Operator.AssignmentOp.None,
                Operator.AssignmentOp.Plus,
                Operator.AssignmentOp.Minus,
                Operator.AssignmentOp.Times -> originalType

                Operator.AssignmentOp.Divide -> ZephyrType.FloatType
                Operator.AssignmentOp.Modulus -> ZephyrType.IntType
                Operator.AssignmentOp.Power -> ZephyrType.FloatType
                Operator.AssignmentOp.And, Operator.AssignmentOp.Or -> ZephyrType.BooleanType
            }.ok()
        }

        is ZephyrExpression.BinaryOpExpr -> {
            val lhsType = lhs.type(env).getOrElse { return it.err() }
            val rhsType = rhs.type(env).getOrElse { return it.err() }


            when (op) {
                Operator.BinaryOp.Plus ->
                    if (lhsType is ZephyrType.StringType && rhsType is ZephyrType.StringType) ZephyrType.StringType
                    else if (lhsType is ZephyrType.FloatType && rhsType is ZephyrType.FloatType) ZephyrType.FloatType
                    else ZephyrType.IntType

                Operator.BinaryOp.Minus,
                Operator.BinaryOp.Times,
                Operator.BinaryOp.Modulus ->
                    if (lhsType is ZephyrType.FloatType && rhsType is ZephyrType.FloatType) ZephyrType.FloatType
                    else ZephyrType.IntType

                Operator.BinaryOp.Divide,
                Operator.BinaryOp.Power -> ZephyrType.FloatType

                Operator.BinaryOp.Equals,
                Operator.BinaryOp.NotEquals,
                Operator.BinaryOp.LessThan,
                Operator.BinaryOp.LessThanEquals,
                Operator.BinaryOp.GreaterThan,
                Operator.BinaryOp.GreaterThanEquals,
                Operator.BinaryOp.And,
                Operator.BinaryOp.Or,
                Operator.BinaryOp.In,
                Operator.BinaryOp.NotIn -> ZephyrType.BooleanType

                Operator.BinaryOp.Range,
                Operator.BinaryOp.RangeInclusive -> ZephyrType.ListType(
                    if (lhsType is ZephyrType.CharType && rhsType is ZephyrType.CharType) ZephyrType.CharType
                    else ZephyrType.IntType

                )
            }.ok()
        }


        is ZephyrExpression.BooleanExpr -> ZephyrType.BooleanType.ok()
        is ZephyrExpression.CallExpr -> when (val funcType = func.type(env).getOrElse { return it.err() }) {
            is ZephyrType.ClosureType -> funcType.returnType.ok()
            else -> ZephyrError.UnexpectedType(
                funcType,
                ZephyrType.ClosureType(emptyList(), ZephyrType.VoidType)
            ).err()
        }

        is ZephyrExpression.CastExpr -> type.ok()
        is ZephyrExpression.CharExpr -> ZephyrType.CharType.ok()
        is ZephyrExpression.ClosureExpr -> ZephyrType.ClosureType(
            params.values.toList(),
            returnType ?: body
                .type(env + params.map { EnvId.Global(it.key) to it.value })
                .getOrElse { return it.err() }
        ).ok()

        is ZephyrExpression.CodeBlockExpr -> body.last().type(env)
        is ZephyrExpression.FloatExpr -> ZephyrType.FloatType.ok()
        is ZephyrExpression.ForExpr -> {
            val iterableType = iterable.type(env).getOrElse { return it.err() }
            val itemType = when (iterableType) {
                is ZephyrType.ListType<*> -> iterableType.itemType
                is ZephyrType.MapType<*, *> -> ZephyrType.TupleType(iterableType.keyType, iterableType.valueType)
                is ZephyrType.ObjectType -> ZephyrType.TupleType(
                    ZephyrType.StringType,
                    ZephyrType.UnionType(iterableType.fields.values.toList())
                )

                ZephyrType.StringType -> ZephyrType.CharType
                is ZephyrType.TupleType -> ZephyrType.UnionType(iterableType.itemTypes)
                else -> return ZephyrError.UnexpectedType(
                    iterableType, ZephyrType.UnionType(
                        ZephyrType.ListType(ZephyrType.VoidType),
                        ZephyrType.MapType(ZephyrType.VoidType, ZephyrType.VoidType),
                        ZephyrType.TupleType(),
                        ZephyrType.ObjectType(emptyMap()),
                        ZephyrType.StringType,
                    )
                ).err()
            }

            body.type(env + (EnvId.Global(name) to itemType))
        }

        is ZephyrExpression.IdentExpr -> env[name]?.ok()
            ?: ZephyrError.IdentNotFound(name, env.keys).err()

        is ZephyrExpression.IfExpr ->
            if (otherwise != null)
                ZephyrType.UnionType(
                    then.type(env).getOrElse { return it.err() },
                    otherwise.type(env).getOrElse { return it.err() }
                ).ok()
            else ZephyrType.VoidType.ok()

        is ZephyrExpression.ImportExpr -> {
            val module = when (path) {
                is ImportPath.File -> findParentWithFile(path.path, "zephyr.config.toml")?.let { projectRoot ->
                    Parser.parse(projectRoot / path.path).getOrElse { return it.err() }
                } ?: return ZephyrError.FileNotFound(path.path).err()

                is ImportPath.Stdlib -> path.module.module
            }

            ZephyrType.ObjectType(
                module.members.filterKeys { idents?.containsKey(it.name) ?: true }.map { (name, data) ->
                    // if there's no "with" clause, use the name as defined
                    // if there is one, check if there's an alias defined
                    (if (idents == null) name.name else idents[name.name] ?: name.name) to data.type
                }.toMap()
            ).ok()
        }

        is ZephyrExpression.IndexAccessExpr -> {
            val parentType = parent.type(env).getOrElse { return it.err() }
            val indexType = index.type(env).getOrElse { return it.err() }
            if (parentType !is ZephyrType.ZephyrIterableType<*, *>) return ZephyrError.UnindexableType(parentType)
                .err()
            if (!(indexType assignableTo parentType.indexType)) return ZephyrError.InvalidIndexType(
                parentType,
                indexType
            ).err()


            return when (parentType) {
                is ZephyrType.ListType<*> -> parentType.itemType
                is ZephyrType.MapType<*, *> -> parentType.valueType
                ZephyrType.StringType -> ZephyrType.CharType

                is ZephyrType.TupleType ->
                    if (index is ZephyrExpression.IntExpr) parentType.itemTypes.getOrNull(index.value)
                        ?: return ZephyrError.InvalidIndexValue(
                            index,
                            parentType,
                            parentType.itemTypes.indices.toList()
                        ).err()
                    else ZephyrType.UnionType(parentType.itemTypes)

                is ZephyrType.ObjectType ->
                    if (index is ZephyrExpression.StringExpr) parentType.fields[index.value]
                        ?: return ZephyrError.InvalidIndexValue(index, parentType, parentType.fields.keys.toList())
                            .err()
                    else ZephyrType.UnionType(parentType.fields.values.toList())
            }.ok()
        }

        is ZephyrExpression.IntExpr -> ZephyrType.IntType.ok()
        is ZephyrExpression.LetExpr -> data.type.ok()
        is ZephyrExpression.ListExpr -> ZephyrType.ListType(value
            .firstOrNull()
            ?.type(env)
            ?.getOrElse { return it.err() }
            ?: ZephyrType.VoidType
        ).ok()

        is ZephyrExpression.ExternalExpr -> data.type.ok()

        is ZephyrExpression.MapExpr -> {
            val (keyType, valueType) = value
                .toList()
                .firstOrNull()
                ?.toList()
                ?.map {
                    it.type(env).getOrElse { e -> return e.err() }
                }
                ?: listOf(ZephyrType.VoidType, ZephyrType.VoidType)

            ZephyrType.MapType(keyType, valueType).ok()
        }

        is ZephyrExpression.MemberAccessExpr -> {
            val parentType = parent.type(env).getOrElse { return it.err() }

            env[member]?.ok() ?: ZephyrError.IdentNotFound(
                member,
                env.keys.filterIsInstance<EnvId.Member>().filter { it.parent assignableTo parentType }
            ).err()
        }

        is ZephyrExpression.ObjectExpr -> ZephyrType.ObjectType(
            value.mapValues { it.value.type }
        ).ok()

        is ZephyrExpression.PostfixOpExpr -> when (op) {
            Operator.PostfixOp.Increment, Operator.PostfixOp.Decrement ->
                if (operand.type(env).getOrElse { return it.err() } == ZephyrType.IntType)
                    ZephyrType.IntType
                else ZephyrType.FloatType
        }.ok()

        is ZephyrExpression.PrefixOpExpr -> when (op) {
            Operator.PrefixOp.Not -> ZephyrType.BooleanType
            Operator.PrefixOp.Negate ->
                if (operand.type(env).getOrElse { return it.err() } == ZephyrType.IntType) ZephyrType.IntType
                else ZephyrType.FloatType
        }.ok()

        is ZephyrExpression.StringExpr -> ZephyrType.StringType.ok()
        is ZephyrExpression.TupleExpr -> ZephyrType.TupleType(value.map {
            it.type(env).getOrElse { e -> return e.err() }
        }).ok()

        is ZephyrExpression.TypeCheckExpr -> ZephyrType.BooleanType.ok()
        is ZephyrExpression.WhileExpr -> ZephyrType.VoidType.ok()
    }
}

infix fun <T : ZephyrType, O : ZephyrType> T.assignableTo(other: O): Boolean {
    // T <=> T
    if (this == other) return true

    // int <=> float
    // string <=> #[char]
    // int <=> char
    when (this to other) {
        Pair(ZephyrType.IntType, ZephyrType.FloatType),
        Pair(ZephyrType.FloatType, ZephyrType.IntType),
        Pair(ZephyrType.StringType, ZephyrType.ListType(ZephyrType.CharType)),
        Pair(ZephyrType.ListType(ZephyrType.CharType), ZephyrType.StringType),
        Pair(ZephyrType.IntType, ZephyrType.CharType), Pair(ZephyrType.CharType, ZephyrType.IntType) -> return true

        else -> {}
    }

    // A | B <=> A | B | ...
    if (this is ZephyrType.UnionType && other is ZephyrType.UnionType) {
        if (this.members.all { other.members.any { o -> it assignableTo o } })
            return true
    }

    // A => A | B | ...
    if (other is ZephyrType.UnionType) {
        if (other.members.any { this assignableTo it }) return true
    }

    // T <=> A, T <=> B -> #{string: T} <=> {a: A, b: B}
    if (this is ZephyrType.MapType<*, *> && other is ZephyrType.ObjectType) {
        if (this.keyType assignableTo ZephyrType.StringType
            && other.fields.values.all { this.valueType assignableTo it }
        ) return true
    }
    if (other is ZephyrType.MapType<*, *> && this is ZephyrType.ObjectType) {
        if (other.keyType assignableTo ZephyrType.StringType
            && this.fields.values.all { other.valueType assignableTo it }
        ) return true
    }

    return false
}
