package com.sarajuhosova.halley.exception.typechecker

import com.sarajuhosova.halley.model.type.HalleyType

class CannotReassignType(
    val expected: HalleyType,
    val actual: HalleyType,
): TypeCheckerException("Expected $expected got $actual")
