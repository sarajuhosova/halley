package com.sarajuhosova.halley.model.value

data class ValueBool(val value: Boolean): HalleyValue() {
    override fun prettyPrint(): String = value.toString()
}
