#include <stdio.h>
#include <stdlib.h>

#include "compiler.h"
#include "scanner.h"

typedef struct {
    Token prev;
    Token curr;
    bool had_error;
    bool panick_mode;
} Parser;

Parser parser;

Chunk* compiling_chunk;

static Chunk* current_chunk() {
    return compiling_chunk;
}

static void error_at(Token* token, const char* message) {
    if (parser.panick_mode) return;
    parser.panick_mode = true;
    
    fprintf(stderr, "[line %d] Error", token-> line);
    
    if (token->type == TOKEN_EOF) {
        fprintf(stderr, " at end");
    } else if (token->type == TOKEN_ERROR) {
        //noop
    } else {
        fprintf(stderr, " at '%.*s'", token->length, token-> start);
    }
    
    fprintf(stderr, ": %s\n", message);
    parser.had_error = true;
}

static void error(const char* message) {
    error_at(&parser.prev, message);
}

static void error_at_current(const char* message) {
    error_at(&parser.curr, message);
}

void advance() {
    parser.prev = parser.curr;
    
    for (;;) {
        parser.curr = scan_token();
        if (parser.curr.type != TOKEN_ERROR) break;
    }
    
    error_at_current(parser.curr.start);
}

void expression() {
    //
}

void consume(TokenType type, const char* message) {
    if (parser.curr.type == type) {
        advance();
        return;
    }
    
    error_at_current(message);
}

static void emit_byte(uint8_t byte) {
    write_chunk(current_chunk(), byte, parser.prev.line);
}

static void emit_bytes(uint8_t byte1, uint8_t byte2) {
    emit_byte(byte1);
    emit_byte(byte2);
}

static void emit_return() {
    emit_byte(OP_RETURN);
}

static void end_compiler() {
    emit_return();
}

bool compile(const char* source, Chunk* chunk) {
    init_scanner(source);
    compiling_chunk = chunk;
    
    parser.had_error = false;
    parser.panick_mode = false;
    
    advance();
    expression();
    consume(TOKEN_EOF, "Expect end of expression.");
    
    end_compiler();
    return !parser.had_error;
}
