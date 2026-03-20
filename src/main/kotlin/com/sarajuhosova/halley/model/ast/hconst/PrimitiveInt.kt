package com.sarajuhosova.halley.model.ast.hconst

class PrimitiveInt(
    val value: Int
): HalleyPrimitive() {
    override fun generate() {
        TODO("Not yet implemented")
    }

    override fun prettyPrint(): String =
        value.toString()
}
