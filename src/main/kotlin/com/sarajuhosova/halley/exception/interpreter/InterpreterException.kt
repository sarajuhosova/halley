package com.sarajuhosova.halley.exception.interpreter

import com.sarajuhosova.halley.exception.HalleyException

abstract class InterpreterException(message: String)
    : HalleyException("[Interpreter] $message")
