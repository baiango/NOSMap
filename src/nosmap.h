#pragma once
#include "vasthash.h"
#include "nosvec.h"


enum NOSMapState {
	NOSMAP_EMPTY = 0,
	NOSMAP_OCCUPIED = 0b1,
	NOSMAP_TOMESTONE = 0b10,
};

typedef struct {
	DataInfo key_info;
	DataInfo value_info;
	size_t capacity;
	float grow_size;
	float load_factor;
} NOSMapNewParameters;

typedef struct {
	size_t index;
	uint64_t hash;
	bool found;
} NOSMapSearchResult;

typedef struct {
	NOSVec key;
	NOSVec value;
} NOSMapKeyValue;

typedef struct {
	DataInfo key_info;
	DataInfo value_info;

	uint8_t *hashes_1_byte;
	NOSVec key_value;
	uint64_t *full_hashes;

	size_t load;
	float grow_size;
	float load_factor;
} NOSMap;

NOSMap nosmap_new(NOSMapNewParameters *params);
NOSMapNewParameters nosmap_default_params();
NOSMapSearchResult nosmap__find_bucket(NOSMap *map, NOSVec *key);
void nosmap_put(NOSMap *map, NOSVec key, NOSVec value);
NOSVec *nosmap_get(NOSMap *map, NOSVec *key);
void nosmap_remove(NOSMap *map, NOSVec *key);
void nosmap__resize(NOSMap *map, size_t new_capacity);

#ifdef TESTS
void nosmap_test();
#endif
