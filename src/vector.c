#ifndef VECTOR
#define VECTOR
#include <stdlib.h>
#include <stdint.h>
#include <stdbool.h>
#include <stdio.h>


enum DataType {
	TYPE_U8,
	TYPE_U16,
	TYPE_U32,
	TYPE_U64,
	TYPE_I8,
	TYPE_I16,
	TYPE_I32,
	TYPE_I64,
	TYPE_F32,
	TYPE_F64,
};

typedef struct {
	void *v;
	uint32_t type;
	uint64_t len;
} Vector;

Vector vector_new(uint64_t len, uint32_t data_size, uint32_t data_type) {
	Vector vec;
	vec.v = malloc(len * data_size);
	if (vec.v == NULL) {
		printf("Failed to allocate address %p!", vec.v);
		return;
	}
	vec.len = len;
	vec.type = TYPE_U32;
	return vec;
}

bool vector_resize(Vector *vec, uint64_t new_len) {
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
void test_vector_new() {
	Vector vec = vector_new(5, sizeof(int), TYPE_U32);
	if (!vec.v || vec.len != 5 || vec.type != TYPE_U32) {
		printf("test_vector_new() test failed!\n");
		return;
	}
	printf("test_vector_new() test passed.\n");
}

void test_vector_resize() {
	Vector vec = vector_new(3, sizeof(int), TYPE_I32);

	bool success = vector_resize(&vec, 7);
	if (!success) {
		printf("test_vector_resize() test failed!\n");
		return;
	}
	printf("test_vector_resize() test passed.\n");
}

void test_vector() {
	test_vector_new();
	test_vector_resize();
	printf("test_vector() all test finished.\n");
}
#endif
#endif
