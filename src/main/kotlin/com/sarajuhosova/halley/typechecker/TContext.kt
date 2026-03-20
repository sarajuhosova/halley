package com.sarajuhosova.halley.typechecker

import com.sarajuhosova.halley.exception.typechecker.CannotReassignType
import com.sarajuhosova.halley.exception.typechecker.VariableNotBound
import com.sarajuhosova.halley.model.type.HalleyType

class TContext {

    private val bindings: MutableMap<String, HalleyType> = mutableMapOf()

    fun bind(name: String, type: HalleyType) {
        if (!bindings.containsKey(name)) {
            bindings[name] = type
        }
        val original = bindings[name]!!
        if (original != type) {
            throw CannotReassignType(original, type)
        }
    }

    fun unbind(name: String) {
        if (!bindings.containsKey(name)) {
            throw VariableNotBound("$name cannot be unbound")
        }
        bindings.remove(name)
    }

    fun get(name: String): HalleyType {
        if (!bindings.containsKey(name)) {
            throw VariableNotBound(name)
        }
        return bindings[name]!!
    }

}
