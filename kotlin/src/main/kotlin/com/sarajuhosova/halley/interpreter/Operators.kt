package com.sarajuhosova.halley.interpreter

import com.sarajuhosova.halley.exception.interpreter.TypeMismatch
import com.sarajuhosova.halley.model.enums.BinOp
import com.sarajuhosova.halley.model.enums.UnOp
import com.sarajuhosova.halley.model.value.HalleyValue
import com.sarajuhosova.halley.model.value.ValueBool
import com.sarajuhosova.halley.model.value.ValueInt
import com.sarajuhosova.halley.model.value.ValueString

object Operators {

    fun unary(op: UnOp, value: HalleyValue): HalleyValue {
        return when (op) {
            UnOp.NOT -> {
                if (value is ValueBool) ValueBool(!value.value)
                    else throw TypeMismatch("Cannot negate a non-boolean")
            }
            UnOp.NEG -> {
                if (value is ValueInt) ValueInt(-value.value)
                    else throw TypeMismatch("Cannot negate a non-integer")
            }
        }
    }

    private fun binaryBool(
        left: HalleyValue, right: HalleyValue,
        op: BinOp,
        operation: (Boolean, Boolean) -> HalleyValue
    ): HalleyValue {
        if (left is ValueBool && right is ValueBool) {
            return operation(left.value, right.value)
        }
        throw TypeMismatch("Cannot apply ${op.prettyPrint()} to non-booleans")
    }

    private fun binaryInt(
        left: HalleyValue, right: HalleyValue,
        op: BinOp,
        operation: (Int, Int) -> HalleyValue
    ): HalleyValue {
        if (left is ValueInt && right is ValueInt) {
            return operation(left.value, right.value)
        }
        throw TypeMismatch("Cannot apply ${op.prettyPrint()} to non-integers")
    }

    fun binary(op: BinOp, left: HalleyValue, right: HalleyValue): HalleyValue {
        if (left.javaClass != right.javaClass) { throw TypeMismatch(
            "Cannot apply ${op.prettyPrint()} to non-matching types: ${left.javaClass} and ${right.javaClass}"
        ) }

        return when (op) {
            BinOp.EQ -> ValueBool(left == right)

            BinOp.LT -> binaryInt(left, right, op) { l, r -> ValueBool(l < r) }
            BinOp.LEQ -> binaryInt(left, right, op) { l, r -> ValueBool(l <= r) }
            BinOp.GT -> binaryInt(left, right, op) { l, r -> ValueBool(l > r) }
            BinOp.GEQ -> binaryInt(left, right, op) { l, r -> ValueBool(l >= r) }

            BinOp.AND -> binaryBool(left, right, op) { l, r -> ValueBool(l && r) }
            BinOp.OR -> binaryBool(left, right, op) { l, r -> ValueBool(l || r) }

            BinOp.ADD -> {
                return when (left) {
                    is ValueInt if right is ValueInt -> {
                        ValueInt(left.value + right.value)
                    }
                    is ValueString if right is ValueString -> {
                        ValueString(left.value + right.value)
                    }
                    else -> throw TypeMismatch("Cannot apply ${op.prettyPrint()} to ${left.javaClass}")
                }
            }
            BinOp.SUB -> binaryInt(left, right, op) { l, r -> ValueInt(l - r) }
            BinOp.MUL -> binaryInt(left, right, op) { l, r -> ValueInt(l * r) }
            BinOp.DIV -> binaryInt(left, right, op) { l, r -> ValueInt(l / r) }
            BinOp.MOD -> binaryInt(left, right, op) { l, r -> ValueInt(l % r) }
        }
    }

}
