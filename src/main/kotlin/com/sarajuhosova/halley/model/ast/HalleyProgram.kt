package com.sarajuhosova.halley.model.ast

import com.sarajuhosova.halley.model.ast.expr.HalleyExpr
import com.sarajuhosova.halley.model.ast.stmt.HalleyStmt

data class HalleyProgram(
    val stmts: List<HalleyStmt>,
    val expr: HalleyExpr
): HalleyElement() {

    override fun prettyPrint(): String =
        stmts.joinToString("\n") { it.prettyPrint() } + expr.prettyPrint()

}
