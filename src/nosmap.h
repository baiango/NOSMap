#pragma once
#include "vasthash.h"
#include "nosvec.h"


typedef struct {
	size_t index;
	bool found;
} NOSMapSearchResult;

typedef struct {
	size_t size;
	NOSVec buckets;
} NOSMap;

NOSMap nosmap_new(size_t initial_capacity, size_t data_size, uint32_t data_type);

#ifdef TESTS
void nosmap_test();
#endif
