#include <stdio.h>
#include <stdlib.h>

#include "common.h"
#include "compiler.h"
#include "memory.h"
#include "scanner.h"

#ifdef DEBUG_PRINT_CODE
#include "debug.h"
#endif

typedef struct {
    Token curr;
    Token prev;
    bool had_error;
    bool panic_mode;
} Parser;

typedef enum {
    PREC_NONE,                      // 0
    PREC_ASSIGNMENT,  // =
    PREC_OR,          // or
    PREC_AND,         // and
    PREC_EQUALITY,    // == !=
    PREC_COMPARISON,  // < > <= >=  // 5
    PREC_TERM,        // + -
    PREC_FACTOR,      // * /
    PREC_UNARY,       // ! -
    PREC_CALL,        // . ()
    PREC_PRIMARY                    // 10
} Precedence;

typedef void (*ParseFn)(bool);

typedef struct {
    ParseFn prefix;
    ParseFn infix;
    Precedence precedence;
} ParseRule;

Parser parser;

Chunk* compiling_chunk;

static Chunk* current_chunk() {
    return compiling_chunk;
}

static void error_at(Token* token, const char* message) {
    if (parser.panic_mode) return;
    parser.panic_mode = true;
    
    fprintf(stderr, "[line %d] Error", token->line);
    
    if (token->type == TOKEN_EOF) {
        fprintf(stderr, " at end");
    } else if (token->type == TOKEN_ERROR) {
        //noop
    } else {
        fprintf(stderr, " at '%.*s'", token->length, token->start);
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
        error_at_current(parser.curr.start);
    }
}

void consume(TokenType type, const char* message) {
    if (parser.curr.type == type) {
        advance();
        return;
    }
    
    error_at_current(message);
}

static bool check(TokenType type) {
    return parser.curr.type == type;
}

static bool match(TokenType type) {
    if (!check(type)) return false;
    advance();
    return true;
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

static uint8_t make_constant(Value val) {
    int constant = add_constant(current_chunk(), val);
    
    if (constant > UINT8_MAX) {
        error("Too many constants in one chunk.");
        return 0;
    }
    
    return (uint8_t)constant;
}

static void emit_constant(Value val) {
    emit_bytes(OP_CONSTANT, make_constant(val));
}

static void end_compiler() {
    emit_return();
#ifdef DEBUG_PRINT_CODE
    if (!parser.had_error) {
        disassemble_chunk(current_chunk(), "code");
    }
#endif
}

static void expression(void);
static void statement(void);
static void declaration(void);
static ParseRule* get_rule(TokenType type);
static void parse_precedence(Precedence presedence);
static uint8_t parse_variable(const char*);
static void define_variable(uint8_t);
static uint8_t identifier_constant(Token*);

static void expression() {
    parse_precedence(PREC_ASSIGNMENT);
}

static void var_declaration() {
    uint8_t global = parse_variable("Expect variable name.");
    
    if (match(TOKEN_EQ)) {
        expression();
    } else {
        emit_byte(OP_NIL);
    }
    
    consume(TOKEN_SEMI, "Expect ';' after variable declaration.");
    
    define_variable(global);
}

static void expression_statement() {
    expression();
    consume(TOKEN_SEMI, "Expect ';' after expression.");
    emit_byte(OP_POP);
}

static void print_statement() {
    expression();
    consume(TOKEN_SEMI, "Expect ';' after value.");
    emit_byte(OP_PRINT);
}

static void synchronize() {
    parser.panic_mode = false;
    
    while (parser.curr.type != TOKEN_EOF) {
        if (parser.prev.type == TOKEN_SEMI) return;
        
        switch(parser.curr.type) {
            case TOKEN_CLASS:
            case TOKEN_FUN:
            case TOKEN_VAR:
            case TOKEN_FOR:
            case TOKEN_IF:
            case TOKEN_WHILE:
            case TOKEN_PRINT:
            case TOKEN_RETURN:
                return;
            default:
                ;
        }
        
        advance();
    }
}

static void declaration() {
    if (match(TOKEN_VAR)) {
        var_declaration();
    } else {
        statement();
    }
    
    if (parser.panic_mode) synchronize();
}

static void statement() {
    if (match(TOKEN_PRINT)) {
        print_statement();
    } else {
        expression_statement();
    }
}

static void binary(bool can_assign) {
    TokenType op_type = parser.prev.type;
    ParseRule* rule = get_rule(op_type);
    
    parse_precedence((Precedence)(rule->precedence + 1));
    
    switch (op_type) {
        case TOKEN_BANG_EQ: emit_bytes(OP_EQUAL, OP_NOT); break;
        case TOKEN_EQ_EQ:   emit_byte(OP_EQUAL); break;
        case TOKEN_GT:      emit_byte(OP_GREATER); break;
        case TOKEN_GT_EQ:   emit_bytes(OP_LESS, OP_NOT); break;
        case TOKEN_LT:      emit_byte(OP_LESS); break;
        case TOKEN_LT_EQ:   emit_bytes(OP_GREATER, OP_NOT); break;
        case TOKEN_PLUS:    emit_byte(OP_ADD); break;
        case TOKEN_MINUS:   emit_byte(OP_SUBTRACT); break;
        case TOKEN_STAR:    emit_byte(OP_MULTIPLY); break;
        case TOKEN_SLASH:   emit_byte(OP_DIVIDE); break;
        default:
            return;
    }
}

static void literal(bool can_assign) {
    switch (parser.prev.type) {
        case TOKEN_FALSE: emit_byte(OP_FALSE); break;
        case TOKEN_NIL: emit_byte(OP_NIL); break;
        case TOKEN_TRUE: emit_byte(OP_TRUE); break;
        default:
            return;
    }
}

static void grouping(bool can_assign) {
    expression();
    consume(TOKEN_RPAREN, "Expect ')' after expression.");
}

static void unary(bool can_assign) {
    TokenType op_type = parser.prev.type;
    
    parse_precedence(PREC_UNARY);
    
    switch (op_type) {
        case TOKEN_BANG: emit_byte(OP_NOT); break;
        case TOKEN_MINUS: emit_byte(OP_NEGATE); break;
        default:
            return;
    }
}

static void number(bool can_assign) {
    double val = strtod(parser.prev.start, NULL);
    emit_constant(NUMBER_VAL(val));
}

static void string(bool can_assign) {
    emit_constant(OBJ_VAL(copy_string(parser.prev.start + 1,
                                      parser.prev.length - 2)));
}

static void named_variable(Token name, bool can_assign) {
    uint8_t arg = identifier_constant(&name);
    
    if (can_assign && match(TOKEN_EQ)) {
        expression();
        emit_bytes(OP_SET_GLOBAL, arg);
    } else {
        emit_bytes(OP_GET_GLOBAL, arg);
    }
}

static void variable(bool can_assign) {
    named_variable(parser.prev, can_assign);
}

ParseRule rules[] = {
    [TOKEN_LPAREN]  = {grouping, NULL,   PREC_NONE},
    [TOKEN_RPAREN]  = {NULL,     NULL,   PREC_NONE},
    [TOKEN_LBRACE]  = {NULL,     NULL,   PREC_NONE},
    [TOKEN_RBRACE]  = {NULL,     NULL,   PREC_NONE},
    [TOKEN_COMMA]   = {NULL,     NULL,   PREC_NONE},
    [TOKEN_DOT]     = {NULL,     NULL,   PREC_NONE},
    [TOKEN_MINUS]   = {unary,    binary, PREC_TERM},
    [TOKEN_PLUS]    = {NULL,     binary, PREC_TERM},
    [TOKEN_SEMI]    = {NULL,     NULL,   PREC_NONE},
    [TOKEN_SLASH]   = {NULL,     binary, PREC_FACTOR},
    [TOKEN_STAR]    = {NULL,     binary, PREC_FACTOR},
    [TOKEN_BANG]    = {unary,    NULL,   PREC_NONE},
    [TOKEN_BANG_EQ] = {NULL,     binary, PREC_EQUALITY},
    [TOKEN_EQ]      = {NULL,     NULL,   PREC_NONE},
    [TOKEN_EQ_EQ]   = {NULL,     binary, PREC_EQUALITY},
    [TOKEN_GT]      = {NULL,     binary, PREC_COMPARISON},
    [TOKEN_GT_EQ]   = {NULL,     binary, PREC_COMPARISON},
    [TOKEN_LT]      = {NULL,     binary, PREC_COMPARISON},
    [TOKEN_LT_EQ]   = {NULL,     binary, PREC_COMPARISON},
    [TOKEN_IDENT]   = {variable, NULL,   PREC_NONE},
    [TOKEN_STR]     = {string,   NULL,   PREC_NONE},
    [TOKEN_NUM]     = {number,   NULL,   PREC_NONE},
    [TOKEN_AND]     = {NULL,     NULL,   PREC_NONE},
    [TOKEN_CLASS]   = {NULL,     NULL,   PREC_NONE},
    [TOKEN_ELSE]    = {NULL,     NULL,   PREC_NONE},
    [TOKEN_FALSE]   = {literal,  NULL,   PREC_NONE},
    [TOKEN_FOR]     = {NULL,     NULL,   PREC_NONE},
    [TOKEN_FUN]     = {NULL,     NULL,   PREC_NONE},
    [TOKEN_IF]      = {NULL,     NULL,   PREC_NONE},
    [TOKEN_NIL]     = {literal,  NULL,   PREC_NONE},
    [TOKEN_OR]      = {NULL,     NULL,   PREC_NONE},
    [TOKEN_PRINT]   = {NULL,     NULL,   PREC_NONE},
    [TOKEN_RETURN]  = {NULL,     NULL,   PREC_NONE},
    [TOKEN_SUPER]   = {NULL,     NULL,   PREC_NONE},
    [TOKEN_THIS]    = {NULL,     NULL,   PREC_NONE},
    [TOKEN_TRUE]    = {literal,  NULL,   PREC_NONE},
    [TOKEN_VAR]     = {NULL,     NULL,   PREC_NONE},
    [TOKEN_WHILE]   = {NULL,     NULL,   PREC_NONE},
    [TOKEN_ERROR]   = {NULL,     NULL,   PREC_NONE},
    [TOKEN_EOF]     = {NULL,     NULL,   PREC_NONE},
};

static void parse_precedence(Precedence precedence) {
    advance();
    
    ParseFn prefix_rule = get_rule(parser.prev.type)->prefix;
    if (prefix_rule == NULL) {
        error("Expect expression.");
        return;
    }
    
    bool can_assign = precedence <= PREC_ASSIGNMENT;
    prefix_rule(can_assign);
    
    while (precedence <= get_rule(parser.curr.type)->precedence) {
        advance();
        ParseFn infix_rule = get_rule(parser.prev.type)->infix;
        infix_rule(can_assign);
    }
    
    if (can_assign && match(TOKEN_EQ)) {
        error("Invalid assignment target.");
    }
}

static uint8_t identifier_constant(Token* name) {
    return make_constant(OBJ_VAL(copy_string(name->start, name->length)));
}

static uint8_t parse_variable(const char* error_message) {
    consume(TOKEN_IDENT, error_message);
    return identifier_constant(&parser.prev);
}

static void define_variable(uint8_t global) {
    emit_bytes(OP_DEFINE_GLOBAL, global);
}

static ParseRule* get_rule(TokenType type) {
    return &rules[type];
}

bool compile(const char* source, Chunk* chunk) {
    init_scanner(source);
    
    compiling_chunk = chunk;
    parser.had_error = false;
    parser.panic_mode = false;
    
    advance();
    
    while (!match(TOKEN_EOF)) {
        declaration();
    }
    
    end_compiler();
    return !parser.had_error;
}
