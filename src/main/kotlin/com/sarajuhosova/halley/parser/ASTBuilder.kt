package com.sarajuhosova.halley.parser

import com.sarajuhosova.halley.HalleyBaseVisitor
import com.sarajuhosova.halley.HalleyParser
import com.sarajuhosova.halley.model.ast.HalleyElement
import com.sarajuhosova.halley.model.ast.HalleyProgram
import com.sarajuhosova.halley.parser.visitor.ExprBuilder
import com.sarajuhosova.halley.parser.visitor.StmtBuilder

object ASTBuilder: HalleyBaseVisitor<HalleyElement>() {

    override fun visitProgram(
        ctx: HalleyParser.ProgramContext
    ): HalleyProgram =
        HalleyProgram(
            ctx.stmt().map { StmtBuilder.visit(it) },
            ExprBuilder.visit(ctx.expr())
        )

}
