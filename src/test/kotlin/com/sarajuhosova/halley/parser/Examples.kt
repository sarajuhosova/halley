package com.sarajuhosova.halley.parser

import com.sarajuhosova.halley.model.ast.hconst.ConstBool
import kotlin.test.Test
import org.assertj.core.api.Assertions.assertThat

class Examples {

    @Test
    fun `test program that only returns true`() {
        val program = """
            true
        """.trimIndent()
        val ast = Parser.parse(program)
        assertThat(ast.stmts).isEmpty()
        assertThat(ast.expr is ConstBool).isTrue()
        val b = ast.expr as ConstBool
        assertThat(b.value).isTrue()
    }

}
