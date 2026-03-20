package com.sarajuhosova.halley.model.ast.hconst

data class ConstString(
    val value: String
): HalleyConst() {
    override fun generate() {
        TODO("Not yet implemented")
    }

    override fun prettyPrint(): String = String.format("\"%s\"", value)
}
