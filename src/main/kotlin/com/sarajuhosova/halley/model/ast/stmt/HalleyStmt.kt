package com.sarajuhosova.halley.model.ast.stmt

import com.sarajuhosova.halley.model.ast.HalleyElement

abstract class HalleyStmt: HalleyElement() {

    protected abstract fun prettyPrintStmt(): String

    override fun prettyPrint(): String = "${prettyPrintStmt()};"

}
