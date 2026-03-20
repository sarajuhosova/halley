package com.sarajuhosova.halley.parser.visitor

import com.sarajuhosova.halley.HalleyBaseVisitor
import com.sarajuhosova.halley.HalleyParser
import com.sarajuhosova.halley.model.ast.stmt.HalleyStmt
import com.sarajuhosova.halley.model.ast.stmt.StmtAssign
import com.sarajuhosova.halley.model.ast.stmt.StmtLet

object StmtBuilder: HalleyBaseVisitor<HalleyStmt>() {

    override fun visitLet(ctx: HalleyParser.LetContext): StmtLet =
        StmtLet(
            ctx.Name().text,
            ExprBuilder.visit(ctx.expr()),
            ctx.Let().text.equals("var")
        )

    override fun visitAssign(ctx: HalleyParser.AssignContext): StmtAssign =
        StmtAssign(
            ctx.Name().text,
            ExprBuilder.visit(ctx.expr())
        )

}
