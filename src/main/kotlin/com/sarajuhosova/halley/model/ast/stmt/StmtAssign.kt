package com.sarajuhosova.halley.model.ast.stmt

import com.sarajuhosova.halley.model.ast.expr.HalleyExpr

data class StmtAssign(
    val name: String,
    val expr: HalleyExpr
): HalleyStmt() {

    override fun prettyPrintStmt(): String =
        "$name = ${expr.prettyPrint()}"

}
