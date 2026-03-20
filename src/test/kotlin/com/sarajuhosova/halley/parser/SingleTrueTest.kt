package com.sarajuhosova.halley.parser

import com.sarajuhosova.halley.model.ast.HalleyProgram
import com.sarajuhosova.halley.model.ast.hconst.ConstBool

class SingleTrueTest: ProgramTest(
    "examples/single_true.hl",
    HalleyProgram(
        emptyList(),
        ConstBool(true)
    )
)
