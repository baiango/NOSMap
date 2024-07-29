#pragma once
#include <stdlib.h>
#include <stdint.h>
#include <stdbool.h>
#include <stdio.h>


enum DataType {
	DATATYPE_UNDEF,
	DATATYPE_U8,
	DATATYPE_U16,
	DATATYPE_U32,
	DATATYPE_U64,
	DATATYPE_I8,
	DATATYPE_I16,
	DATATYPE_I32,
	DATATYPE_I64,
	DATATYPE_F32,
	DATATYPE_F64,
};

typedef struct {
	void *v;
	uint64_t len;
	uint32_t type;
} NOSVec;

void nosvec_destroy(NOSVec* vec);
NOSVec nosvec_new(uint64_t len, uint32_t data_size, uint32_t data_type);
bool nosvec_resize(NOSVec *vec, uint64_t new_len);

#ifdef TESTS
void nosvec_test_new();
void nosvec_test_resize();
void nosvec_test();
#endif

