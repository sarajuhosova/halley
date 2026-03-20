package com.sarajuhosova.halley.model.ast.expr

data class ExprVar(
    val name: String
): HalleyExpr() {
    override fun generate() {
        TODO("Not yet implemented")
    }

    override fun prettyPrint(): String = name
}
