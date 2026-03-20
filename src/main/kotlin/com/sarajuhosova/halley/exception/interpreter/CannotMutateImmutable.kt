package com.sarajuhosova.halley.exception.interpreter

class CannotMutateImmutable(val name: String)
    : InterpreterException("Cannot mutate immutable variable $name")
