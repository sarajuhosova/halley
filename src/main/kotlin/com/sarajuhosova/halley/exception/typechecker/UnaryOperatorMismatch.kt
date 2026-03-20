package com.sarajuhosova.halley.exception.typechecker

import com.sarajuhosova.halley.model.enums.UnOp
import com.sarajuhosova.halley.model.type.HalleyType

class UnaryOperatorMismatch(
    op: UnOp, expected: HalleyType, actual: HalleyType
): TypeCheckerException("The $op operator expects a $expected, you provided a $actual")
