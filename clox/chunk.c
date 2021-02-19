#include <stdlib.h>

#include "chunk.h"
#include "memory.h"

void init_chunk(Chunk* chunk) {
    chunk->count = 0;
    chunk->cap = 0;
    chunk->lines = NULL;
    chunk->code = NULL;
    
    init_value_array(&chunk->constants);
}

void free_chunk(Chunk* chunk) {
    FREE_ARRAY(uint8_t, chunk->code, chunk->cap);
    FREE_ARRAY(int, chunk->lines, chunk->cap);
    free_value_array(&chunk->constants);
    
    init_chunk(chunk);
}

void write_chunk(Chunk* chunk, uint8_t byte, int line) {
    if (chunk->cap < chunk->count + 1) {
        int old_cap = chunk->cap;
        
        chunk->cap = GROW_CAPACITY(old_cap);
        chunk->code = GROW_ARRAY(uint8_t, chunk->code, old_cap, chunk->cap);
        chunk->lines = GROW_ARRAY(int, chunk->lines, old_cap, chunk->cap);
    }
    
    chunk->code[chunk->count] = byte;
    chunk->lines[chunk->count] = line;
    chunk->count++;
}

int add_constant(Chunk* chunk, Value val) {
    write_value_array(&chunk->constants, val);
    return chunk->constants.count - 1;
}


