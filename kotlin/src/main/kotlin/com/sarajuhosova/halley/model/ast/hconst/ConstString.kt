package com.sarajuhosova.halley.model.ast.hconst

import com.sarajuhosova.halley.model.type.HalleyString
import com.sarajuhosova.halley.model.value.ValueString

data class ConstString(
    val value: String
): ExprConst(ValueString(value), HalleyString) {

    override fun prettyPrint(): String = "\"$value\""

}
