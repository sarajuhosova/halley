package com.sarajuhosova.halley.model.ast.hconst

import com.sarajuhosova.halley.model.value.ValueString

data class ConstString(
    val value: String
): ExprConst(ValueString(value)) {

    override fun prettyPrint(): String = "\"$value\""

}
