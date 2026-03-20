package com.sarajuhosova.halley.model.value

data class ValueInt(val value: Int): HalleyValue() {
    override fun prettyPrint(): String = value.toString()
}
