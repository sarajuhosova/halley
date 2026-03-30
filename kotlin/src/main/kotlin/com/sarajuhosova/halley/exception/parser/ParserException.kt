package com.sarajuhosova.halley.exception.parser

import com.sarajuhosova.halley.exception.HalleyException

abstract class ParserException(message: String)
    : HalleyException("[Parser] $message")
