package com.theonlytails.zephyr_lang

import com.github.ajalt.mordant.rendering.TextColors
import com.theonlytails.zephyr_lang.parser.*
import com.theonlytails.zephyr_lang.stdlib.StdlibModule
import com.willowtreeapps.fuzzywuzzy.diffutils.FuzzySearch
import okio.Path

internal data class MultiException(inline val errors: Collection<Throwable>) :
    Throwable(errors.joinToString("\n") { it.message ?: "" })

sealed class ZephyrError(message: String) : Throwable(TextColors.red(message)) {
    data class UnexpectedType(val found: ZephyrType, val expected: ZephyrType) :
        ZephyrError("""Expected value to be of type $expected, found $found""")

    data class IdentNotFound(val ident: EnvId, val options: Iterable<EnvId>) :
        ZephyrError(
            buildString {
                append("Could not find an identifier named \"${ident.name}\"")
                if (ident is EnvId.Member) append(" on parent type ${ident.parent}")
                append(".")

                FuzzySearch.extractOne(
                    ident.name,
                    options.map(EnvId::toString)
                ).string?.let { append(""" Did you mean "$it"?""") }
            }
        )

    data class UnindexableType(val found: ZephyrType) : ZephyrError(
        """Cannot index into a value of type $found, expected a string, a list, a map, a tuple, or an object."""
    )

    data class InvalidIndexType(val iterableType: ZephyrType.ZephyrIterableType<*, *>, val indexType: ZephyrType) :
        ZephyrError(
            """Cannot index into value of type $iterableType with a value of type $indexType, expected ${iterableType.indexType}"""
        )

    data class InvalidIndexValue(
        val actual: ZephyrExpression.ZephyrLiteralExpr<*>,
        val parent: ZephyrType.ZephyrIterableType<*, *>,
        val validIndices: Iterable<*>
    ) : ZephyrError(
        """Cannot index into a value of type $parent using the literal "${actual.value}". 
            |Try one of these valid indices: ${validIndices.joinToString()} """.trimMargin()
    )

    data class DuplicateIdent(val name: Name) :
        ZephyrError("""An identifier named "$name" already exists in this scope. Please rename your declaration.""")

    data class InvalidOperation(val op: Operator, val lhs: ZephyrType, val rhs: ZephyrType?) :
        ZephyrError("Cannot apply the $op (${op.humanName})" + (if (op is Operator.AssignmentOp) " assignment" else "") + " operator to these operands: $lhs $op $rhs") {
        constructor(op: Operator, operand: ZephyrType) : this(op, operand, null)
    }

    data class AssignmentToImmutable(val ident: EnvId) : ZephyrError("Cannot assign a new value to the immutable variable $ident")

    data class NonClosureCall(val found: ZephyrType) :
        ZephyrError("Values of type $found cannot be called as closures.")

    data class WrongParamType(val found: ZephyrType, val expected: ZephyrType) :
        ZephyrError("Cannot pass a value of type $found into a closure argument of type $expected")

    data class FileNotFound(val path: Path) : ZephyrError("Cannot parse file at $path.")

    data class InvalidStdlibModule(val found: String) :
        ZephyrError(buildString {
            append("A module named \"")
            append(found)
            append("\" does not exist in the standard library.")
            FuzzySearch.extractOne(
                found,
                StdlibModule.entries.map { it.moduleName }
            ).string?.let { append(""" Did you mean "$it"?""") }
        })
}
