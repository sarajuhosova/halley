package com.sarajuhosova.halley.typechecker

import com.sarajuhosova.halley.exception.typechecker.BinaryOperatorMismatch
import com.sarajuhosova.halley.exception.typechecker.UnaryOperatorMismatch
import com.sarajuhosova.halley.model.enums.BinOp
import com.sarajuhosova.halley.model.enums.UnOp
import com.sarajuhosova.halley.model.type.HalleyBool
import com.sarajuhosova.halley.model.type.HalleyInt
import com.sarajuhosova.halley.model.type.HalleyType

object Operator {

    fun unary(op: UnOp, actual: HalleyType): HalleyType {
        fun check(expected: HalleyType, produced: HalleyType): HalleyType {
            if (actual == expected) return produced
            throw UnaryOperatorMismatch(op, expected, actual)
        }

        return when (op) {
            UnOp.NOT -> check(HalleyBool, HalleyBool)
            UnOp.NEG -> check(HalleyInt, HalleyInt)
        }
    }

    fun binary(op: BinOp, left: HalleyType, right: HalleyType): HalleyType {
        when (op) {
            BinOp.EQ -> {
                if (left == right) return left
                throw BinaryOperatorMismatch(
                    op,
                    Pair(left, right),
                    Pair(left, left)
                )
            }

            BinOp.LT -> TODO()
            BinOp.LEQ -> TODO()
            BinOp.GT -> TODO()
            BinOp.GEQ -> TODO()

            BinOp.AND -> TODO()
            BinOp.OR -> TODO()

            BinOp.ADD -> TODO()
            BinOp.SUB -> TODO()
            BinOp.MUL -> TODO()
            BinOp.DIV -> TODO()
            BinOp.MOD -> TODO()
        }
    }

}
