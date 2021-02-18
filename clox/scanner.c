#include <stdio.h>
#include <string.h>

#include "common.h"
#include "scanner.h"

typedef struct {
    const char* start;
    const char* curr;
    int line;
} Scanner;

Scanner scanner;

void init_scanner(const char* source) {
    scanner.start = source;
    scanner.curr = source;
    scanner.line = 1;
}

static bool is_digit(char c) {
    return c >= '0' && c <= '9';
}

static bool is_lower(char c) {
    return c >= 'a' && c <= 'z';
}

static bool is_upper(char c) {
    return c >= 'A' && c <= 'Z';
}

static bool is_alpha(char c) {
    return is_lower(c) || is_upper(c) || c == '_';
}

static bool at_end() {
    return *scanner.curr == '\0';
}

static Token make_token(TokenType type) {
    Token token;
    token.type = type;
    token.start = scanner.start;
    token.length = (int)(scanner.curr - scanner.start);
    token.line = scanner.line;
    
    return token;
}

static Token error_token(const char* message) {
    Token token;
    token.type = TOKEN_ERROR;
    token.start = message;
    token.length = (int)strlen(message);
    token.line = scanner.line;
    
    return token;
}

static char advance() {
    return *scanner.curr++;
}

static bool match(char expected) {
    if (at_end()) return false;
    if (*scanner.curr != expected) return false;
    
    scanner.curr++;
    return true;
}

static char peek() {
    return *scanner.curr;
}

static char peek_next() {
    if (at_end()) return '\0';
    return scanner.curr[1];
}

static void skip_whitespace() {
    for (;;) {
        char c = peek();
        
        switch (c) {
            case ' ':
            case '\r':
            case '\t':
                advance();
                break;
            case '\n':
                scanner.line++;
                advance();
                break;
            case '/':
                if (peek_next() == '/') {
                    while (peek() != '\n' && !at_end()) advance();
                } else {
                    return;
                }
                break;
            default:
                return;
        }
    }
}

static Token string() {
    while (peek() != '"' && !at_end()) {
        if (peek() == '\n') scanner.line++;
        advance();
    }
    
    if (at_end()) return error_token("Unterminated string.");
    
    advance();
    
    return make_token(TOKEN_STR);
}

static Token number() {
    while (is_digit(peek())) advance();
    
    if (peek() == '.' && is_digit(peek_next())) {
        advance();
        
        while (is_digit(peek())) advance();
    }
    
    return make_token(TOKEN_NUM);
}

static TokenType check_kw(int start, int len, const char* rest, TokenType tt) {
    if (scanner.curr - scanner.start == start + len &&
        memcmp(scanner.start + start, rest, len) == 0) {
        
        return tt;
    }
    
    return TOKEN_IDENT;
    
}

static TokenType identifier_type() {
    switch (scanner.start[0]) {
        case 'a': return check_kw(1, 2, "nd", TOKEN_AND);
        case 'c': return check_kw(1, 4, "lass", TOKEN_CLASS);
        case 'e': return check_kw(1, 3, "lse", TOKEN_ELSE);
        case 'f':
            if (scanner.curr - scanner.start > 1) {
                switch (scanner.start[1]) {
                    case 'a': return check_kw(2, 3, "lse", TOKEN_FALSE);
                    case 'o': return check_kw(2, 1, "r", TOKEN_FOR);
                    case 'u': return check_kw(2, 1, "n", TOKEN_FUN);
                }
            }
            break;
        case 'i': return check_kw(1, 1, "f", TOKEN_IF);
        case 'n': return check_kw(1, 2, "il", TOKEN_NIL);
        case 'o': return check_kw(1, 1, "r", TOKEN_OR);
        case 'p': return check_kw(1, 4, "rint", TOKEN_PRINT);
        case 'r': return check_kw(1, 5, "eturn", TOKEN_RETURN);
        case 's': return check_kw(1, 4, "uper", TOKEN_SUPER);
        case 't':
            if (scanner.curr - scanner.start > 1) {
                switch (scanner.start[1]) {
                    case 'h': return check_kw(2, 2, "is", TOKEN_THIS);
                    case 'r': return check_kw(2, 2, "ue", TOKEN_TRUE);
                }
            }
            break;
        case 'v': return check_kw(1, 2, "ar", TOKEN_VAR);
        case 'w': return check_kw(1, 4, "hile", TOKEN_WHILE);
    }
    
    return TOKEN_IDENT;
}

static Token identifier() {
    while (is_alpha(peek()) || is_digit(peek())) advance();
    
    return make_token(identifier_type());
}

Token scan_token() {
    skip_whitespace();
    
    scanner.start = scanner.curr;
    if (at_end()) return make_token(TOKEN_EOF);
    
    char c = advance();
    
    if (is_alpha(c)) return identifier();
    if (is_digit(c)) return number();
    
    switch (c) {
        case '(' : return make_token(TOKEN_LPAREN);
        case ')' : return make_token(TOKEN_RPAREN);
        case '{' : return make_token(TOKEN_LBRACE);
        case '}' : return make_token(TOKEN_RBRACE);
        case ';' : return make_token(TOKEN_SEMI);
        case ',' : return make_token(TOKEN_COMMA);
        case '.' : return make_token(TOKEN_DOT);
        case '-' : return make_token(TOKEN_MINUS);
        case '+' : return make_token(TOKEN_PLUS);
        case '/' : return make_token(TOKEN_SLASH);
        case '*' : return make_token(TOKEN_STAR);
        case '!': return make_token(match('=') ? TOKEN_BANG_EQ : TOKEN_BANG);
        case '=': return make_token(match('=') ? TOKEN_EQ_EQ : TOKEN_EQ);
        case '<': return make_token(match('=') ? TOKEN_LT_EQ : TOKEN_LT);
        case '>': return make_token(match('=') ? TOKEN_GT_EQ : TOKEN_GT);
        case '"': return string();
    }
    
    return error_token("Unexpected character.");
}
