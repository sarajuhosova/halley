package com.sarajuhosova.halley.model.ast.hconst

import com.sarajuhosova.halley.model.value.ValueBool

data class ConstBool(
    val value: Boolean
): ExprConst(ValueBool(value)) {

    override fun prettyPrint(): String =
        if (value) "true" else "false"

}
