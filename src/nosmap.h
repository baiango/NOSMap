#pragma once


typedef struct {
	uint64_t index;
	bool found;
} NOSMapSearchResult;

typedef struct {
	uint64_t size;
	NOSVec buckets;
} NOSMap;

NOSMap nosmap_new(uint64_t initial_capacity, uint32_t data_size, uint32_t data_type);
#ifdef TESTS
void nosmap_test();
#endif
