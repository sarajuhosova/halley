package com.sarajuhosova.halley.model.ast.hconst

import com.sarajuhosova.halley.model.ast.expr.HalleyExpr
import com.sarajuhosova.halley.model.type.HalleyType
import com.sarajuhosova.halley.model.value.HalleyValue

sealed class ExprConst(
    val hValue: HalleyValue,
    val type: HalleyType
): HalleyExpr()
