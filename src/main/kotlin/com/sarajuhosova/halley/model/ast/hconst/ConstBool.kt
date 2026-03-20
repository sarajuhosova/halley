package com.sarajuhosova.halley.model.ast.hconst

class ConstBool(
    val value: Boolean
): HalleyConst() {
    override fun generate() {
        TODO("Not yet implemented")
    }

    override fun prettyPrint(): String =
        if (value) "true" else "false"
}
