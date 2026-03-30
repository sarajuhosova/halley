package com.sarajuhosova.halley.exception.interpreter

import com.sarajuhosova.halley.model.ast.HalleyElement

class NotImplemented(clazz: Class<HalleyElement>): InterpreterException(
    "Cannot deal with ${clazz.name}, sorry"
)
