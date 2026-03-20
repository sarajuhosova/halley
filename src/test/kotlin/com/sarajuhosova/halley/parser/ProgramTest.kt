package com.sarajuhosova.halley.parser

import com.sarajuhosova.halley.model.ast.HalleyProgram
import org.assertj.core.api.Assertions.assertThat
import kotlin.test.Test

abstract class ProgramTest(
    file: String,
    val AST: HalleyProgram
) {

    val program: String = this::class.java.classLoader.getResourceAsStream(file)!!
        .bufferedReader().readLines().joinToString("\n")

    @Test
    fun `parse program`() {
        val ast = Parser.parse(program)
        assertThat(ast).isEqualTo(AST)
    }

}
