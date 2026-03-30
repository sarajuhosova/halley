package com.sarajuhosova.halley.exception.typechecker

import com.sarajuhosova.halley.exception.HalleyException

abstract class TypeCheckerException(message: String)
    : HalleyException("[TypeChecker] $message")
