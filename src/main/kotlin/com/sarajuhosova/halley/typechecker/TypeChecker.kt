package com.sarajuhosova.halley.typechecker

import com.sarajuhosova.halley.exception.typechecker.NotImplemented
import com.sarajuhosova.halley.model.ast.HalleyProgram
import com.sarajuhosova.halley.model.ast.expr.ExprBinOp
import com.sarajuhosova.halley.model.ast.expr.ExprUnOp
import com.sarajuhosova.halley.model.ast.expr.ExprVar
import com.sarajuhosova.halley.model.ast.expr.HalleyExpr
import com.sarajuhosova.halley.model.ast.hconst.ExprConst
import com.sarajuhosova.halley.model.ast.stmt.HalleyStmt
import com.sarajuhosova.halley.model.ast.stmt.StmtAssign
import com.sarajuhosova.halley.model.ast.stmt.StmtLet
import com.sarajuhosova.halley.model.type.HalleyInt
import com.sarajuhosova.halley.model.type.HalleyType

object TypeChecker {

    private fun typecheck(ctx: TContext, expr: HalleyExpr): HalleyType {
        when (expr) {
            is ExprConst -> {
                return expr.type
            }
            is ExprVar -> {
                return ctx.get(expr.name)
            }
            is ExprUnOp -> {
                val type = typecheck(ctx, expr.expr)
                return Operator.unary(expr.operator, type)
            }
            is ExprBinOp -> {
                val left = typecheck(ctx, expr.left)
                val right = typecheck(ctx, expr.right)
                return Operator.binary(expr.operator, left, right)
            }
            else -> throw NotImplemented(expr.javaClass)
        }
        return HalleyInt
    }

    private fun typecheck(ctx: TContext, stmt: HalleyStmt) {
        when (stmt) {
            is StmtLet -> {
                val type = typecheck(ctx, stmt.expr)
                ctx.bind(stmt.name, type)
            }
            is StmtAssign -> {
                val type = typecheck(ctx, stmt.expr)
                ctx.bind(stmt.name, type)
            }
            else -> throw NotImplemented(stmt.javaClass)
        }
    }

    fun typecheck(program: HalleyProgram): HalleyType {
        val ctx = TContext()

        for (s in program.stmts) {
            typecheck(ctx, s)
        }
        return typecheck(ctx, program.expr)
    }

}
