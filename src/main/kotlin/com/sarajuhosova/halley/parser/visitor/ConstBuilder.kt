package com.sarajuhosova.halley.parser.visitor

import com.sarajuhosova.halley.HalleyBaseVisitor
import com.sarajuhosova.halley.HalleyParser
import com.sarajuhosova.halley.model.ast.hconst.ConstBool
import com.sarajuhosova.halley.model.ast.hconst.ConstInt
import com.sarajuhosova.halley.model.ast.hconst.ConstString
import com.sarajuhosova.halley.model.ast.hconst.ExprConst

object ConstBuilder: HalleyBaseVisitor<ExprConst>() {

    override fun visitBool(ctx: HalleyParser.BoolContext): ConstBool =
        ConstBool(ctx.text.toBoolean())

    override fun visitInt(ctx: HalleyParser.IntContext): ConstInt =
        ConstInt(ctx.text.toInt())

    override fun visitString(ctx: HalleyParser.StringContext): ConstString =
        ConstString(ctx.text)

}
