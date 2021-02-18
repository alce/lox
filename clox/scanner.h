#ifndef clox_scanner_h
#define clox_scanner_h

typedef enum {
    TOKEN_LPAREN,
    TOKEN_RPAREN,
    TOKEN_LBRACE,
    TOKEN_RBRACE,
    TOKEN_COMMA,
    TOKEN_DOT, // 5
    TOKEN_MINUS,
    TOKEN_PLUS,
    TOKEN_SEMI,
    TOKEN_SLASH,
    TOKEN_STAR, // 10
    
    TOKEN_BANG,
    TOKEN_BANG_EQ,
    TOKEN_EQ,
    TOKEN_EQ_EQ,
    TOKEN_GT, // 15
    TOKEN_GT_EQ,
    TOKEN_LT,
    TOKEN_LT_EQ,
    
    // Literals.
    TOKEN_IDENT,
    TOKEN_STR, //20
    TOKEN_NUM,
    
    // Keywords.
    TOKEN_AND,
    TOKEN_CLASS,
    TOKEN_ELSE,
    TOKEN_FALSE, // 25
    TOKEN_FOR,
    TOKEN_FUN,
    TOKEN_IF,
    TOKEN_NIL,
    TOKEN_OR, // 30
    TOKEN_PRINT,
    TOKEN_RETURN,
    TOKEN_SUPER,
    TOKEN_THIS,
    TOKEN_TRUE, // 35
    TOKEN_VAR,
    TOKEN_WHILE,
    
    TOKEN_ERROR, // 38
    TOKEN_EOF    // 39
} TokenType;

typedef struct {
    TokenType type;
    const char* start;
    int line;
    int length;
} Token;

void init_scanner(const char* source);

Token scan_token(void);

#endif
