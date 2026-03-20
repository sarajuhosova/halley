package com.sarajuhosova.halley.exception.interpreter

class VariableNotBound(
    cause: String
): InterpreterException("Variable doesn't exist: $cause")
