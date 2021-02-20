#include <stdio.h>

#include "debug.h"
#include "value.h"


void disassemble_chunk(Chunk* chunk, const char* name){
    printf("== %s ==\n", name);
    
    for (int offset = 0; offset < chunk->count;) {
        offset = disassemble_instruction(chunk, offset);
    }
}

static int simple(const char* name, int offset) {
    printf("%s\n", name);
    return offset + 1;
}

static int byte(const char* name, Chunk* chunk, int offset) {
    uint8_t slot = chunk->code[offset + 1];
    printf("%-16s %4d\n", name, slot);
    
    return offset + 2;
}

static int constant(const char* name, Chunk* chunk, int offset) {
    uint8_t constant = chunk->code[offset + 1];
    
    printf("%-16s %4d '", name, constant);
    print_value(chunk->constants.values[constant]);
    printf("'\n");
    
    return offset + 2;
}

int disassemble_instruction(Chunk* chunk, int offset) {
    printf("%04d ", offset);
    
    if (offset > 0 &&
        chunk->lines[offset] == chunk->lines[offset - 1]) {
        printf("   | ");
    } else {
        printf("%4d ", chunk->lines[offset]);
    }
    
    uint8_t instruction = chunk->code[offset];
    
    switch (instruction) {
        case OP_CONSTANT: return constant("OP_CONSTANT", chunk, offset);
        case OP_NIL: return simple("OP_NIL", offset);
        case OP_TRUE: return simple("OP_TRUE", offset);
        case OP_FALSE: return simple("OP_FALSE", offset);
        case OP_POP: return simple("OP_POP", offset);
        case OP_GET_LOCAL: return byte("OP_GET_LOCAL", chunk, offset);
        case OP_SET_LOCAL: return byte("OP_SET_LOCAL", chunk, offset);
        case OP_GET_GLOBAL: return constant("OP_GET_GLOBAL", chunk, offset);
        case OP_DEFINE_GLOBAL:
            return constant("OP_DEFINE_GLOBAL", chunk, offset);
        case OP_SET_GLOBAL: return constant("OP_SET_GLOBAL", chunk, offset);
        case OP_EQUAL: return simple("OP_EQUAL", offset);
        case OP_GREATER: return simple("OP_GREATER", offset);
        case OP_LESS: return simple("OP_LESS", offset);
        case OP_ADD: return simple("OP_ADD", offset);
        case OP_SUBTRACT: return simple("OP_SUBTRACT", offset);
        case OP_MULTIPLY: return simple("OP_MULTIPLY", offset);
        case OP_DIVIDE: return simple("OP_DIVIDE", offset);
        case OP_NOT: return simple("OP_NOT", offset);
        case OP_NEGATE: return simple("OP_NEGATE", offset);
        case OP_PRINT: return simple("OP_PRINT", offset);
        case OP_RETURN: return simple("OP_RETURN", offset);
        default:
            printf("Unknown opcode %d\n", instruction);
            return offset + 1 ;
    }
}

