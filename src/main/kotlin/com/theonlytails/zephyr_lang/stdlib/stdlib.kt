package com.theonlytails.zephyr_lang.stdlib

import com.theonlytails.zephyr_lang.parser.*
import com.theonlytails.zephyr_lang.parser.ZephyrType.*

enum class StdlibModule(val moduleName: String, val module: ZephyrModule) {
    Math(
        "math", ZephyrModule(
            mapOf(
                EnvId.Global("e") to Declaration.Actual(FloatType, ZephyrExpression.FloatExpr(2.718281828)),
                EnvId.Global("pi") to Declaration.Actual(FloatType, ZephyrExpression.FloatExpr(3.141592653)),
                EnvId.Global("sqrt2") to Declaration.Actual(
                    FloatType,
                    ZephyrExpression.FloatExpr(1.414213562)
                ),
                EnvId.Global("sin") to Declaration.Unknown(
                    ClosureType(listOf(FloatType), FloatType)
                ),
                EnvId.Global("sinh") to Declaration.Unknown(
                    ClosureType(listOf(FloatType), FloatType)
                ),
                EnvId.Global("asin") to Declaration.Unknown(
                    ClosureType(listOf(FloatType), FloatType)
                ),
                EnvId.Global("asinh") to Declaration.Unknown(
                    ClosureType(listOf(FloatType), FloatType)
                ),
                EnvId.Global("cos") to Declaration.Unknown(
                    ClosureType(listOf(FloatType), FloatType)
                ),
                EnvId.Global("cosh") to Declaration.Unknown(
                    ClosureType(listOf(FloatType), FloatType)
                ),
                EnvId.Global("acos") to Declaration.Unknown(
                    ClosureType(listOf(FloatType), FloatType)
                ),
                EnvId.Global("acosh") to Declaration.Unknown(
                    ClosureType(listOf(FloatType), FloatType)
                ),
                EnvId.Global("tan") to Declaration.Unknown(
                    ClosureType(listOf(FloatType), FloatType)
                ),
                EnvId.Global("tanh") to Declaration.Unknown(
                    ClosureType(listOf(FloatType), FloatType)
                ),
                EnvId.Global("atan") to Declaration.Unknown(
                    ClosureType(listOf(FloatType), FloatType)
                ),
                EnvId.Global("atanh") to Declaration.Unknown(
                    ClosureType(listOf(FloatType), FloatType)
                ),
                EnvId.Global("atan2") to Declaration.Unknown(
                    ClosureType(listOf(FloatType), FloatType)
                ),
                EnvId.Global("abs") to Declaration.Unknown(
                    ClosureType(listOf(FloatType), FloatType)
                ),
                EnvId.Global("sign") to Declaration.Unknown(
                    ClosureType(listOf(FloatType), IntType)
                ),
                EnvId.Global("sqrt") to Declaration.Unknown(
                    ClosureType(listOf(FloatType), FloatType)
                ),
                EnvId.Global("cbrt") to Declaration.Unknown(
                    ClosureType(listOf(FloatType), FloatType)
                ),
                EnvId.Global("cbrt") to Declaration.Unknown(
                    ClosureType(listOf(FloatType), FloatType)
                ),
                EnvId.Global("floor") to Declaration.Unknown(
                    ClosureType(listOf(FloatType), IntType)
                ),
                EnvId.Global("ceil") to Declaration.Unknown(
                    ClosureType(listOf(FloatType), IntType)
                ),
                EnvId.Global("round") to Declaration.Unknown(
                    ClosureType(listOf(FloatType), IntType)
                ),
                EnvId.Global("trunc") to Declaration.Unknown(
                    ClosureType(listOf(FloatType), IntType)
                ),
                EnvId.Global("min") to Declaration.Unknown(
                    ClosureType(listOf(FloatType, FloatType), FloatType)
                ),
                EnvId.Global("max") to Declaration.Unknown(
                    ClosureType(listOf(FloatType, FloatType), FloatType)
                ),
                EnvId.Global("clamp") to Declaration.Unknown(
                    ClosureType(listOf(FloatType, FloatType, FloatType), FloatType)
                ),
            )
        )
    );
//    FS("fs"),
//    RESULT("result", ),
//    RAND("rand", ),

    companion object {
        fun fromString(moduleName: String) = entries.find { it.moduleName == moduleName }
    }
}
