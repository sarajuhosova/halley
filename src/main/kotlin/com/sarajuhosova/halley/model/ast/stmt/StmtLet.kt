package com.sarajuhosova.halley.model.ast.stmt

import com.sarajuhosova.halley.model.ast.expr.HalleyExpr

class StmtLet(
    val name: String,
    val expr: HalleyExpr,
    val mut: Boolean = false
): HalleyStmt() {

    fun getBinder(): String = if (mut) "var" else "val"

    override fun generate() {
        TODO("Not yet implemented")
    }

    override fun prettyPrintStmt(): String =
        String.format("%s %s = %s", getBinder(), name, expr.prettyPrint())

}
