package com.sarajuhosova.halley.parser

import com.sarajuhosova.halley.model.ast.HalleyProgram
import com.sarajuhosova.halley.model.ast.hconst.ConstBool
import com.sarajuhosova.halley.model.value.ValueBool

class SingleTrueTest: ProgramTest(
    "examples/single_true.hl",
    HalleyProgram(
        emptyList(),
        ConstBool(true)
    ),
    ValueBool(true)
)
