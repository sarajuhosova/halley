package com.sarajuhosova.halley.model.value

abstract class HalleyValue {

    abstract fun prettyPrint(): String

    override fun toString(): String = prettyPrint()

}
