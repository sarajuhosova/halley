package com.sarajuhosova.halley.model.ast.hconst

import com.sarajuhosova.halley.model.ast.expr.HalleyExpr
import com.sarajuhosova.halley.model.value.HalleyValue

abstract class ExprConst(val hValue: HalleyValue): HalleyExpr()
