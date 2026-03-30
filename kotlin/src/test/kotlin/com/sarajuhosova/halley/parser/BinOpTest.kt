package com.sarajuhosova.halley.parser

import com.sarajuhosova.halley.model.ast.HalleyProgram
import com.sarajuhosova.halley.model.ast.expr.ExprBinOp
import com.sarajuhosova.halley.model.ast.expr.ExprVar
import com.sarajuhosova.halley.model.ast.hconst.ConstBool
import com.sarajuhosova.halley.model.ast.stmt.StmtAssign
import com.sarajuhosova.halley.model.ast.stmt.StmtLet
import com.sarajuhosova.halley.model.enums.BinOp
import com.sarajuhosova.halley.model.value.ValueBool

class BinOpTest: ProgramTest(
    "examples/binop.hl",
    HalleyProgram(
        listOf(
            StmtLet("x", ConstBool(true), true),
            StmtAssign("x", ExprBinOp(BinOp.AND, ExprVar("x"), ConstBool(false))),
        ),
        ExprVar("x")
    ),
    ValueBool(false)
)
