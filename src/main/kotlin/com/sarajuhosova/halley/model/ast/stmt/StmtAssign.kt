package com.sarajuhosova.halley.model.ast.stmt

import com.sarajuhosova.halley.model.ast.expr.HalleyExpr

class StmtAssign(
    val name: String,
    val expr: HalleyExpr
): HalleyStmt() {

    override fun generate() {
        TODO("Not yet implemented")
    }

    override fun prettyPrintStmt(): String =
        String.format("%s = %s", name, expr.prettyPrint())

}
