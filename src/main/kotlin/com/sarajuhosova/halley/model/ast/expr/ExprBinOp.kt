package com.sarajuhosova.halley.model.ast.expr

class ExprBinOp(
    val operator: String,
    val left: HalleyExpr,
    val right: HalleyExpr,
): HalleyExpr() {
    override fun generate() {
        TODO("Not yet implemented")
    }

    override fun prettyPrint(): String =
        left.prettyPrint() + operator + right.prettyPrint()
}
