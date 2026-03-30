package com.sarajuhosova.halley.exception.typechecker

class VariableNotBound(
    cause: String
): TypeCheckerException("Variable does not exist: $cause")
