#include <stdio.h>

#include "memory.h"
#include "value.h"

void init_value_array(ValueArray* array) {
    array->cap = 0;
    array->count = 0;
    array->values = NULL;
}

void write_value_array(ValueArray* array, Value value) {
    if (array->cap < array->count + 1) {
        int old_cap = array->cap;
        array->cap = GROW_CAPACITY(old_cap);
        array->values = GROW_ARRAY(Value, array->values, old_cap, array->cap);
    }
    
    array->values[array->count] = value;
    array->count++;
}

void free_value_array(ValueArray* array) {
    FREE_ARRAY(Value, array->values, array->cap);
    init_value_array(array);
}

void print_value(Value value) {
    switch (value.type) {
        case VAL_BOOL:
            printf(AS_BOOL(value) ? "true": "false");
            break;
        case VAL_NIL: printf("nil"); break;
        case VAL_NUMBER: printf("%g", AS_NUMBER(value)); break;
    }
}
