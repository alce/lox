#ifndef clox_debug_h
#define clox_debug_h

#include "chunk.h"

void dissasemble_chunk(Chunk* chunk, const char* name);
int dissasemble_instruction(Chunk* chunk, int offset);

#endif 
