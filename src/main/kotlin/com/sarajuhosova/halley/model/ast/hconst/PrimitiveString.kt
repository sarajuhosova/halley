package com.sarajuhosova.halley.model.ast.hconst

class PrimitiveString(
    val value: String
): HalleyPrimitive() {
    override fun generate() {
        TODO("Not yet implemented")
    }

    override fun prettyPrint(): String = String.format("\"%s\"", value)
}
