package com.sarajuhosova.halley.exception.typechecker

import com.sarajuhosova.halley.model.ast.HalleyElement

class NotImplemented(clazz: Class<HalleyElement>): TypeCheckerException(
    "Cannot deal with ${clazz.name}, sorry"
)
