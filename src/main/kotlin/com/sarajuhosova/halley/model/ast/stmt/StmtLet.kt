package com.sarajuhosova.halley.model.ast.stmt

import com.sarajuhosova.halley.model.ast.expr.HalleyExpr

data class StmtLet(
    val name: String,
    val expr: HalleyExpr,
    val mutable: Boolean = false
): HalleyStmt() {

    fun getBinder(): String = if (mutable) "var" else "val"

    override fun prettyPrintStmt(): String =
        "${getBinder()} $name = ${expr.prettyPrint()}"

}
