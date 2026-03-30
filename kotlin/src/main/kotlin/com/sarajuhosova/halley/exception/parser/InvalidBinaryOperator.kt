package com.sarajuhosova.halley.exception.parser

class InvalidBinaryOperator(op: String)
    : ParserException("Invalid unary operator $op")
