package com.sarajuhosova.halley.exception.typechecker

import com.sarajuhosova.halley.model.enums.BinOp
import com.sarajuhosova.halley.model.type.HalleyType

class BinaryOperatorMismatch(
    op: BinOp, expected: Pair<HalleyType, HalleyType>, actual: Pair<HalleyType, HalleyType>
): TypeCheckerException(
    "The $op operator expects a ${expected.first} and ${expected.second}, " +
            "you provided a ${actual.first} and ${actual.second}"
)
