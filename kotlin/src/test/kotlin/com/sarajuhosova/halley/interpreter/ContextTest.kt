package com.sarajuhosova.halley.interpreter

import com.sarajuhosova.halley.exception.interpreter.CannotMutateImmutable
import com.sarajuhosova.halley.exception.interpreter.VariableNotBound
import org.assertj.core.api.Assertions.assertThat
import org.assertj.core.api.Assertions.assertThatThrownBy
import kotlin.test.Test

class ContextTest {

    @Test
    fun `test empty throws`() {
        val context = Context<Int>()

        assertThatThrownBy { context.get("x") }
            .isInstanceOf(VariableNotBound::class.java)
    }

    @Test
    fun `test get unbound throws`() {
        val context = Context<Int>()
        context.bind("x", 0)


        assertThatThrownBy { context.get("y") }
            .isInstanceOf(VariableNotBound::class.java)
    }

    @Test
    fun `test bind-get`() {
        val context = Context<Int>()
        context.bind("x", 0)

        assertThat(context.get("x")).isEqualTo(0)
    }

    @Test
    fun `test bind-unbind-get throws`() {
        val context = Context<Int>()
        context.bind("x", 0)
        context.unbind("x")

        assertThatThrownBy { context.get("x") }
            .isInstanceOf(VariableNotBound::class.java)
    }

    @Test
    fun `test bind-bind-get`() {
        val context = Context<Int>()
        context.bind("x", 0)
        context.bind("x", 1)

        assertThat(context.get("x")).isEqualTo(1)
    }

    @Test
    fun `test bind-bind-unbind-get`() {
        val context = Context<Int>()
        context.bind("x", 0)
        context.bind("x", 1)
        context.unbind("x")

        assertThat(context.get("x")).isEqualTo(0)
    }

    @Test
    fun `test bind immutable is immutable`() {
        val context = Context<Int>()
        context.bind("x", 0)

        assertThat(context.isMutable("x")).isFalse()
    }

    @Test
    fun `test bind mutable is mutable`() {
        val context = Context<Int>()
        context.bind("x", 0, true)

        assertThat(context.isMutable("x")).isTrue()
    }

    @Test
    fun `mutability check for unbound throws`() {
        val context = Context<Int>()

        assertThatThrownBy { context.isMutable("x") }
            .isInstanceOf(VariableNotBound::class.java)
    }

    @Test
    fun `test mutate immutable throws`() {
        val context = Context<Int>()
        context.bind("x", 0)

        assertThatThrownBy { context.mutate("x", 1) }
            .isInstanceOf(CannotMutateImmutable::class.java)
    }

    @Test
    fun `test bind-mutate-get`() {
        val context = Context<Int>()
        context.bind("x", 0, true)
        context.mutate("x", 1)

        assertThat(context.get("x")).isEqualTo(1)
    }

    @Test
    fun `test bind-bind-mutate-get`() {
        val context = Context<Int>()
        context.bind("x", 0)
        context.bind("x", 1, true)
        context.mutate("x", 2)

        assertThat(context.get("x")).isEqualTo(2)
    }

    @Test
    fun `test bind-mutate-unbind-get throws`() {
        val context = Context<Int>()
        context.bind("x", 0, true)
        context.mutate("x", 1)
        context.unbind("x")

        assertThatThrownBy { context.get("x") }
            .isInstanceOf(VariableNotBound::class.java)
    }

    @Test
    fun `test bind-bind-mutate-unbind-get returns first`() {
        val context = Context<Int>()
        context.bind("x", 0)
        context.bind("x", 1, true)
        context.mutate("x", 2)
        context.unbind("x")

        assertThat(context.get("x")).isEqualTo(0)
    }

}
