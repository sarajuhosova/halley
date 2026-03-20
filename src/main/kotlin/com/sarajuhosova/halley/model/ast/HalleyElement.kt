package com.sarajuhosova.halley.model.ast

abstract class HalleyElement {

    abstract fun generate()

    abstract fun prettyPrint(): String

}
