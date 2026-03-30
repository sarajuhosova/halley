package com.sarajuhosova.halley.parser

import com.sarajuhosova.halley.interpreter.Interpreter
import com.sarajuhosova.halley.model.ast.HalleyProgram
import com.sarajuhosova.halley.model.value.HalleyValue
import org.assertj.core.api.Assertions.assertThat
import kotlin.test.Test

abstract class ProgramTest(
    file: String,
    val AST: HalleyProgram,
    val VAL: HalleyValue
) {

    val program: String = this::class.java.classLoader.getResourceAsStream(file)!!
        .bufferedReader().readLines().joinToString("\n")

    @Test
    fun `parse program`() {
        val ast = Parser.parse(program)
        assertThat(ast).isEqualTo(AST)
    }

    @Test
    fun `interpret program`() {
        val value = Interpreter.interpret(Parser.parse(program))
        assertThat(value).isEqualTo(VAL)
    }

}
