package com.sarajuhosova.halley.model.enums

enum class UnOp(
    val operator: String
) {
    NOT("!"),
    NEG("-");

    fun prettyPrint(): String = this.operator

    companion object {
        val map = entries.associateBy(UnOp::operator)

        fun from(operator: String): UnOp? = map[operator]
    }
}
