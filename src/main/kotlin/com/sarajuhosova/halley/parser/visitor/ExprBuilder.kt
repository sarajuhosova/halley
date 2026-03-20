package com.sarajuhosova.halley.parser.visitor

import com.sarajuhosova.halley.HalleyBaseVisitor
import com.sarajuhosova.halley.HalleyParser
import com.sarajuhosova.halley.model.ast.expr.ExprBinOp
import com.sarajuhosova.halley.model.ast.expr.ExprUnOp
import com.sarajuhosova.halley.model.ast.expr.ExprVar
import com.sarajuhosova.halley.model.ast.expr.HalleyExpr
import com.sarajuhosova.halley.model.ast.hconst.HalleyPrimitive

object ExprBuilder: HalleyBaseVisitor<HalleyExpr>() {

    override fun visitPrim(ctx: HalleyParser.PrimContext): HalleyPrimitive {
        return PrimitiveBuilder.visit(ctx)
    }

    override fun visitVar(ctx: HalleyParser.VarContext): ExprVar =
        ExprVar(ctx.text)

    override fun visitUnop(ctx: HalleyParser.UnopContext): ExprUnOp =
        ExprUnOp(ctx.op.text, visit(ctx.e))

    override fun visitBinop(ctx: HalleyParser.BinopContext): ExprBinOp =
        ExprBinOp(
            ctx.op.text,
            visit(ctx.left),
            visit(ctx.right)
        )

}
