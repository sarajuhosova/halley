package com.sarajuhosova.halley.exception.parser

class InvalidUnaryOperator(op: String)
    : ParserException("Invalid unary operator $op")
