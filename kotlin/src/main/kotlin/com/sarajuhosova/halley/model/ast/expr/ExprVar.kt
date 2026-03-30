package com.sarajuhosova.halley.model.ast.expr

data class ExprVar(
    val name: String
): HalleyExpr() {

    override fun prettyPrint(): String = name

}
