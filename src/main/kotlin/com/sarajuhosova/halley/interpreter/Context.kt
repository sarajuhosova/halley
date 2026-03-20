package com.sarajuhosova.halley.interpreter

import com.sarajuhosova.halley.exception.interpreter.CannotMutateImmutable
import com.sarajuhosova.halley.exception.interpreter.VariableNotBound

class Context<V> {

    private val bindings: MutableMap<String, MutableList<Pair<V, Boolean>>>
        = mutableMapOf()

    fun bind(name: String, value: V, mutable: Boolean = false) {
        val entry = Pair(value, mutable)
        if (!bindings.containsKey(name)) {
            bindings[name] = mutableListOf(entry)
        } else {
            bindings[name]!!.add(entry)
        }
    }

    fun isMutable(name: String): Boolean {
        if (!bindings.containsKey(name)) {
            throw VariableNotBound(name)
        }

        return bindings[name]!!.last().second
    }

    fun mutate(name: String, value: V) {
        if (!bindings.containsKey(name)) {
            throw VariableNotBound("$name cannot be mutated")
        }

        val values = bindings[name]!!
        val entry = values[values.lastIndex]
        if (!entry.second) {
            throw CannotMutateImmutable(name)
        }
        values[values.lastIndex] = Pair(value, true)
    }

    fun unbind(name: String) {
        if (!bindings.containsKey(name)) {
            throw VariableNotBound("$name cannot be unbound")
        }

        val values = bindings[name]!!
        if (values.size < 2) {
            bindings.remove(name)
        } else {
            values.removeLast()
        }
    }

    fun get(name: String): V {
        if (!bindings.containsKey(name)) {
            throw VariableNotBound(name)
        }
        return bindings[name]!!.last().first
    }

}
