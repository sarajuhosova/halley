package com.sarajuhosova.halley.model.ast.expr

import com.sarajuhosova.halley.model.enums.UnOp

data class ExprUnOp(
    val operator: UnOp,
    val expr: HalleyExpr
): HalleyExpr() {

    override fun prettyPrint(): String =
        operator.prettyPrint() + expr.prettyPrint()

}
