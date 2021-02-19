#include <stdlib.h>
#include <string.h>

#include "memory.h"
#include "object.h"
#include "table.h"
#include "value.h"

#define TABLE_MAX_LOAD 0.75

void init_table(Table* table) {
    table->count = 0;
    table->cap = 0;
    table->entries = NULL;
}

void free_table(Table* table) {
    FREE_ARRAY(Entry, table->entries, table->cap);
    init_table(table);
}

static Entry* find_entry(Entry* entries, int cap, ObjString* key) {
    uint32_t idx = key->hash % cap;
    Entry* tombstone = NULL;
    
    for (;;) {
        Entry* entry = &entries[idx];
        
        if (entry->key == NULL) {
            if (IS_NIL(entry->value)) {
                return tombstone != NULL ? tombstone : entry;
            } else {
                if (tombstone == NULL) tombstone = entry;
            }
        } else if (entry->key == key) {
            return entry;
        }
        
        idx = (idx + 1) % cap;
    }
}

bool table_get(Table* table, ObjString* key, Value* value) {
    if (table->count == 0) return false;
    
    Entry* entry = find_entry(table->entries, table->cap, key);
    if (entry->key == NULL) return false;
    
    *value = entry->value;
    return true;
}

static void adjust_capacity(Table* table, int cap) {
    Entry* entries = ALLOCATE(Entry, cap);
    
    for (int i = 0; i < cap; i++) {
        entries[i].key = NULL;
        entries[i].value = NIL_VAL;
    }
    
    // re-build the table
    table->count = 0;
    for (int i = 0; i < table->cap; i++) {
        Entry* entry = &table->entries[i];
        if (entry->key == NULL) continue;
        
        Entry* dest = find_entry(entries, cap, entry->key);
        dest->key = entry->key;
        dest->value = entry-> value;
        table->count++;
    }
    
    FREE_ARRAY(Entry, table->entries, table->cap);
    
    table->entries = entries;
    table->count = cap;
}

bool table_set(Table* table, ObjString* key, Value value) {
    if (table->count + 1 > table->cap * TABLE_MAX_LOAD) {
        int cap = GROW_CAPACITY(table->cap);
        adjust_capacity(table, cap);
    }
    
    Entry* entry = find_entry(table->entries, table->cap, key);
    
    bool is_new = entry->key == NULL;
    if (is_new && IS_NIL(entry->value)) table->count++;
    
    entry->key = key;
    entry->value = value;
    return is_new;
}

bool table_delete(Table* table, ObjString* key) {
    if (table->count == 0) return false;
    
    Entry* entry = find_entry(table->entries, table->cap, key);
    if (entry->key == NULL) return false;
    
    // toombstone
    entry->key = NULL;
    entry->value = BOOL_VAL(true);
    
    return true;
}

void table_add_all(Table* from, Table* to) {
    for (int i = 0; i < from->cap; i++) {
        Entry* entry = &from->entries[i];
        if (entry->key != NULL) {
            table_set(to, entry->key, entry->value);
        }
    }
}

ObjString* table_find_string(Table* table, const char* chars,
                             int length, uint32_t hash) {
    if (table->count == 0) return NULL;
    uint32_t idx = hash % table->cap;
    
    for (;;) {
        Entry* entry = &table->entries[idx];
        
        if (entry->key == NULL) {
            if (IS_NIL(entry->value)) return NULL;
        } else if (entry->key->length == length
                   && entry->key->hash == hash
                   && memcmp(entry->key->chars, chars, length) == 0) {
            return entry->key;
        }
        
        idx = (idx + 1) % table->cap;
    }
}



