#pragma once
#include "vasthash.h"
#include "nosvec.h"


typedef struct {
	size_t index;
	bool found;
} NOSMapSearchResult;

typedef struct {
	void *key;
	void *value;
} NOSMapKeyValue;

typedef struct {
	size_t size;
	uint8_t *hashes_1_byte;
	NOSVec key_value;
} NOSMap;

NOSMap nosmap_new(size_t initial_capacity, size_t data_size, uint32_t data_type);

#ifdef TESTS
void nosmap_test();
#endif
