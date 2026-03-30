package com.sarajuhosova.halley.parser

import com.sarajuhosova.halley.model.ast.HalleyProgram
import com.sarajuhosova.halley.model.ast.expr.ExprUnOp
import com.sarajuhosova.halley.model.ast.expr.ExprVar
import com.sarajuhosova.halley.model.ast.hconst.ConstBool
import com.sarajuhosova.halley.model.ast.stmt.StmtLet
import com.sarajuhosova.halley.model.enums.UnOp
import com.sarajuhosova.halley.model.value.ValueBool

class SingleReturnTest: ProgramTest(
    "examples/single_return.hl",
    HalleyProgram(
        listOf(
            StmtLet("b", ConstBool(true))
        ),
        ExprUnOp(UnOp.NOT, ExprVar("b"))
    ),
    ValueBool(false)
)
