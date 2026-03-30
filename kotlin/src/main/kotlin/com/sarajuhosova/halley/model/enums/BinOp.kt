package com.sarajuhosova.halley.model.enums

enum class BinOp(
    val operator: String
) {
    EQ("=="),

    LT("<"),
    LEQ("<="),
    GT(">"),
    GEQ(">="),

    AND("&&"),
    OR("||"),

    ADD("+"),
    SUB("-"),
    MUL("*"),
    DIV("/"),
    MOD("%")
    ;

    fun prettyPrint(): String = this.operator

    companion object {
        val map = entries.associateBy(BinOp::operator)

        fun from(operator: String): BinOp? = map[operator]
    }
}
