package com.sarajuhosova.halley.interpreter

import com.sarajuhosova.halley.exception.interpreter.NotImplemented
import com.sarajuhosova.halley.model.ast.HalleyProgram
import com.sarajuhosova.halley.model.ast.expr.ExprBinOp
import com.sarajuhosova.halley.model.ast.expr.ExprUnOp
import com.sarajuhosova.halley.model.ast.expr.ExprVar
import com.sarajuhosova.halley.model.ast.expr.HalleyExpr
import com.sarajuhosova.halley.model.ast.hconst.ExprConst
import com.sarajuhosova.halley.model.ast.stmt.HalleyStmt
import com.sarajuhosova.halley.model.ast.stmt.StmtAssign
import com.sarajuhosova.halley.model.ast.stmt.StmtLet
import com.sarajuhosova.halley.model.value.HalleyValue

object Interpreter {

    private fun interpret(
        ctx: Context<HalleyValue>,
        expr: HalleyExpr
    ): HalleyValue {
        when (expr) {
            is ExprConst -> {
                return expr.hValue
            }
            is ExprVar -> {
                return ctx.get(expr.name)
            }
            is ExprUnOp -> {
                val value = interpret(ctx, expr.expr)
                return Operators.unary(expr.operator, value)
            }
            is ExprBinOp -> {
                val left = interpret(ctx, expr.left)
                val right = interpret(ctx, expr.right)
                return Operators.binary(expr.operator, left, right)
            }
            else -> throw NotImplemented(expr.javaClass)
        }
    }

    private fun interpret(
        ctx: Context<HalleyValue>,
        stmt: HalleyStmt
    ) {
        when (stmt) {
            is StmtLet -> {
                val value = interpret(ctx, stmt.expr)
                ctx.bind(stmt.name, value, stmt.mutable)
            }
            is StmtAssign -> {
                val value = interpret(ctx, stmt.expr)
                ctx.mutate(stmt.name, value)
            }
            else -> throw NotImplemented(stmt.javaClass)
        }
    }

    fun interpret(program: HalleyProgram): HalleyValue {
        val ctx = Context<HalleyValue>()

        for (s in program.stmts) {
            interpret(ctx, s)
        }
        return interpret(ctx, program.expr)
    }

}
