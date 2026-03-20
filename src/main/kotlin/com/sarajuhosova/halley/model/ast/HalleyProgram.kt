package com.sarajuhosova.halley.model.ast

import com.sarajuhosova.halley.model.ast.expr.HalleyExpr
import com.sarajuhosova.halley.model.ast.stmt.HalleyStmt

class HalleyProgram(
    val stmts: List<HalleyStmt> = emptyList(),
    val expr: HalleyExpr? = null
): HalleyElement() {
    override fun generate() {
        TODO("Not yet implemented")
    }

    override fun prettyPrint(): String {
        val body: String = stmts.joinToString("\n") { it.prettyPrint() }
        return if (expr != null) body + expr.prettyPrint() else body
    }
}
