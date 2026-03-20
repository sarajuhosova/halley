package com.sarajuhosova.halley.model.ast.expr

data class ExprUnOp(
    val operator: String,
    val expr: HalleyExpr
): HalleyExpr() {
    override fun generate() {
        TODO("Not yet implemented")
    }

    override fun prettyPrint(): String =
        operator + expr.prettyPrint()
}
