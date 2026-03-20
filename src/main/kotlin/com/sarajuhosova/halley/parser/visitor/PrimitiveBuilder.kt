package com.sarajuhosova.halley.parser.visitor

import com.sarajuhosova.halley.HalleyBaseVisitor
import com.sarajuhosova.halley.HalleyParser
import com.sarajuhosova.halley.model.ast.hconst.HalleyPrimitive
import com.sarajuhosova.halley.model.ast.hconst.PrimitiveBool
import com.sarajuhosova.halley.model.ast.hconst.PrimitiveInt
import com.sarajuhosova.halley.model.ast.hconst.PrimitiveString

object PrimitiveBuilder: HalleyBaseVisitor<HalleyPrimitive>() {

    override fun visitBool(ctx: HalleyParser.BoolContext): PrimitiveBool =
        PrimitiveBool(ctx.text.toBoolean())

    override fun visitInt(ctx: HalleyParser.IntContext): PrimitiveInt =
        PrimitiveInt(ctx.text.toInt())

    override fun visitString(ctx: HalleyParser.StringContext): PrimitiveString =
        PrimitiveString(ctx.text)

}
