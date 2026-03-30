package com.sarajuhosova.halley.model.ast.expr

import com.sarajuhosova.halley.model.enums.BinOp

data class ExprBinOp(
    val operator: BinOp,
    val left: HalleyExpr,
    val right: HalleyExpr,
): HalleyExpr() {

    override fun prettyPrint(): String =
        left.prettyPrint() + operator.prettyPrint() + right.prettyPrint()

}
