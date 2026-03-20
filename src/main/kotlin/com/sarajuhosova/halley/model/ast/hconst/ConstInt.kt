package com.sarajuhosova.halley.model.ast.hconst

import com.sarajuhosova.halley.model.value.ValueInt

data class ConstInt(
    val value: Int
): ExprConst(ValueInt(value)) {

    override fun prettyPrint(): String = value.toString()

}
