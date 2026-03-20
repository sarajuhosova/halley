package com.sarajuhosova.halley.parser.visitor

import com.sarajuhosova.halley.HalleyBaseVisitor
import com.sarajuhosova.halley.HalleyParser
import com.sarajuhosova.halley.exception.parser.InvalidBinaryOperator
import com.sarajuhosova.halley.exception.parser.InvalidUnaryOperator
import com.sarajuhosova.halley.model.ast.expr.ExprBinOp
import com.sarajuhosova.halley.model.ast.expr.ExprUnOp
import com.sarajuhosova.halley.model.ast.expr.ExprVar
import com.sarajuhosova.halley.model.ast.expr.HalleyExpr
import com.sarajuhosova.halley.model.ast.hconst.ExprConst
import com.sarajuhosova.halley.model.enums.BinOp
import com.sarajuhosova.halley.model.enums.UnOp

object ExprBuilder: HalleyBaseVisitor<HalleyExpr>() {

    override fun visitConst(ctx: HalleyParser.ConstContext): ExprConst {
        return ConstBuilder.visit(ctx)
    }

    override fun visitVar(ctx: HalleyParser.VarContext): ExprVar =
        ExprVar(ctx.text)

    override fun visitUnop(ctx: HalleyParser.UnopContext): ExprUnOp {
        val op = UnOp.from(ctx.op.text)
            ?: throw InvalidUnaryOperator(ctx.op.text)
        return ExprUnOp(op, visit(ctx.e))
    }

    override fun visitBinop(ctx: HalleyParser.BinopContext): ExprBinOp {
        val op = BinOp.from(ctx.op.text)
            ?: throw InvalidBinaryOperator(ctx.op.text)
        return ExprBinOp(
            op,
            visit(ctx.left),
            visit(ctx.right)
        )
    }

}
