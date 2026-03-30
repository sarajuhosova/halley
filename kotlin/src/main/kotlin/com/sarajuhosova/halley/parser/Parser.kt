package com.sarajuhosova.halley.parser

import com.sarajuhosova.halley.HalleyLexer
import com.sarajuhosova.halley.HalleyParser
import com.sarajuhosova.halley.model.ast.HalleyProgram
import org.antlr.v4.runtime.CharStream
import org.antlr.v4.runtime.CharStreams
import org.antlr.v4.runtime.CommonTokenStream

object Parser {

    fun parse(input: String): HalleyProgram =
        parse(CharStreams.fromString(input))

    fun parse(stream: CharStream): HalleyProgram {
        val lexer = HalleyLexer(stream)
        val tokens = CommonTokenStream(lexer)
        val parser = HalleyParser(tokens)
        val context = parser.program()

        return ASTBuilder.visitProgram(context)
    }

}
