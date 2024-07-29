#pragma once
#include <stdlib.h>
#include <stdint.h>
#include <stdbool.h>
#include <stdio.h>


enum DataType {
	DATATYPE_UNDEF,
	DATATYPE_PTR = 1 << 31,
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
	DATATYPE_SIZE,
	DATATYPE_M256I,
	DATATYPE_NOSMAP_KEY_VALUE
};


typedef struct {
	uint32_t size;
	uint32_t type;
} DataInfo;

typedef struct {
	void *v;
	size_t len;
	DataInfo info;
} NOSVec;

void nosvec_destroy(NOSVec* vec);
NOSVec nosvec_new(size_t len, DataInfo data_info);
bool nosvec_resize(NOSVec *vec, size_t new_len);

#ifdef TESTS
void nosvec_test_new();
void nosvec_test_resize();
void nosvec_test();
#endif
