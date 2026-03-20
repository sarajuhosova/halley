package com.sarajuhosova.halley.model.ast.hconst

class PrimitiveBool(
    val value: Boolean
): HalleyPrimitive() {
    override fun generate() {
        TODO("Not yet implemented")
    }

    override fun prettyPrint(): String =
        if (value) "true" else "false"
}
