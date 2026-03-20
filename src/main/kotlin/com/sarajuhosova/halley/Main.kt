package com.sarajuhosova.halley

import com.sarajuhosova.halley.parser.Parser

fun main() {
    println("Halley!")

    val example = """
        true
    """.trimIndent()

    val ast = Parser.parse(example)

    println(ast.prettyPrint())
}
