package com.sarajuhosova.halley.model.value

data class ValueString(val value: String): HalleyValue() {
    override fun prettyPrint(): String = "\"$value\""
}
