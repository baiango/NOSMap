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
	DataInfo key_info;
	DataInfo value_info;
	uint8_t *hashes_1_byte;
	NOSVec key_value;
} NOSMap;

NOSMap nosmap_new(size_t initial_capacity, DataInfo key_info, DataInfo value_type);
NOSMapSearchResult nosmap__find_bucket(NOSMap *map, NOSVec *key);

#ifdef TESTS
void nosmap_test();
#endif
