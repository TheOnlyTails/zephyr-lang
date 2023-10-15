package com.theonlytails.zephyr_lang.parser

sealed class EnvId(open val name: String) {
    data class Global(override val name: String) : EnvId(name) {
        override fun toString() = name
    }

    data class Member(override val name: String, val parent: ZephyrType) : EnvId(name) {
        override fun toString() = "$parent.$name"
    }
}

typealias ParserEnv = MutableMap<EnvId, Declaration>

fun ParserEnv.toTypeMap() = mapValues { it.value.type }
