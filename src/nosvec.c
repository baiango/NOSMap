#ifndef NOSVEC
#define NOSVEC
#include <stdlib.h>
#include <stdint.h>
#include <stdbool.h>
#include <stdio.h>


enum DataType {
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
	uint32_t type;
	uint64_t len;
} NOSVec;

__attribute__((destructor)) void nosvec_destroy(NOSVec* vec) {
#if !defined(__OPTIMIZE__) || (__OPTIMIZE__ == 0)
	printf("Destroyed NOSVec #%p!", vec);
#endif
	free(vec->v);
	vec->v = NULL;
}

NOSVec nosvec_new(uint64_t len, uint32_t data_size, uint32_t data_type) {
	NOSVec vec = {NULL, data_type, len};
	vec.v = malloc(len * data_size);
	if (vec.v == NULL) {
		printf("Failed to allocate address %p!", vec.v);
		return vec;
	}
	return vec;
}

bool nosvec_resize(NOSVec *vec, uint64_t new_len) {
	void *new_data = realloc(vec->v, new_len * sizeof(uint32_t));
	if (new_data == NULL) {
		printf("Failed to resize address %p!", new_data);
		return false;
	}
	vec->v = new_data;
	vec->len = new_len;
	return true;
}

#ifdef TESTS
void nosvec_test_new() {
	NOSVec vec = nosvec_new(5, sizeof(int), DATATYPE_U32);
	if (!vec.v || vec.len != 5 || vec.type != DATATYPE_U32) {
		printf("nosvec_test_new() test failed!\n");
		return;
	}
	printf("nosvec_test_new() test passed.\n");
}

void nosvec_test_resize() {
	NOSVec vec = nosvec_new(3, sizeof(int), DATATYPE_I32);

	bool success = nosvec_resize(&vec, 7);
	if (!success) {
		printf("nosvec_test_resize() test failed!\n");
		return;
	}
	printf("nosvec_test_resize() test passed.\n");
}

void nosvec_test() {
	nosvec_test_new();
	nosvec_test_resize();
	printf("nosvec_test() all test finished.\n");
}
#endif
#endif
