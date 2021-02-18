#include "fs.h"

#include <stdio.h>
#include <stdlib.h>


char* read_file(const char* path) {
    FILE* file = fopen(path, "rb");
    
    if (file == NULL) {
        fprintf(stderr, "Failed to open file \"%s\".\n", path);
        exit(74);
    }
    
    fseek(file, 0L, SEEK_END);
    size_t fileSize = ftell(file);
    rewind(file);
    
    char* buffer = (char*)malloc(fileSize + 1);
    if (buffer == NULL) {
        fprintf(stderr, "OOM. Can't read \"%s\".\n", path);
        exit(74);
    }
    
    size_t bytesRead = fread(buffer, sizeof(char), fileSize, file);
    buffer[bytesRead] = '\0';
    if (bytesRead < fileSize) {
        fprintf(stderr, "Failed to read file \"%s\".\n", path);
        exit(74);
    }
    
    fclose(file);
    return buffer;
}
