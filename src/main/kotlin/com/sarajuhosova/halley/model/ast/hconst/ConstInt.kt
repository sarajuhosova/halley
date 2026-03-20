package com.sarajuhosova.halley.model.ast.hconst

class ConstInt(
    val value: Int
): HalleyConst() {
    override fun generate() {
        TODO("Not yet implemented")
    }

    override fun prettyPrint(): String =
        value.toString()
}
