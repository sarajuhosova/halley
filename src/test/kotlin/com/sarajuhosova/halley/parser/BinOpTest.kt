package com.sarajuhosova.halley.parser

import com.sarajuhosova.halley.model.ast.HalleyProgram
import com.sarajuhosova.halley.model.ast.expr.ExprBinOp
import com.sarajuhosova.halley.model.ast.expr.ExprVar
import com.sarajuhosova.halley.model.ast.hconst.ConstBool
import com.sarajuhosova.halley.model.ast.stmt.StmtAssign
import com.sarajuhosova.halley.model.ast.stmt.StmtLet

class BinOpTest: ProgramTest(
    "examples/binop.hl",
    HalleyProgram(
        listOf(
            StmtLet("x", ConstBool(true), true),
            StmtAssign("x", ExprBinOp("&&", ExprVar("x"), ConstBool(false))),
        ),
        ExprVar("x")
    )
)
