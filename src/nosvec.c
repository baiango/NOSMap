#include "nosvec.h"


__attribute__((destructor)) void nosvec_destroy(NOSVec* vec) {
#if !defined(__OPTIMIZE__) || (__OPTIMIZE__ == 0)
	printf("nosvec_destroy | Destroyed NOSVec #%p!", vec);
#endif
	free(vec->v);
}

NOSVec nosvec_new(size_t len, size_t data_size, uint32_t data_type) {
	NOSVec vec = {NULL, 0, DATATYPE_UNDEF};
	vec.v = malloc(len * data_size);
	if (vec.v == NULL) {
		printf("nosvec_new | Failed to allocate address %p!", vec.v);
		return vec;
	}
	vec.len = len;
	vec.data_size = data_size;
	vec.data_type = data_type;
	return vec;
}

bool nosvec_resize(NOSVec *vec, size_t new_len) {
	void *new_data = realloc(vec->v, new_len * vec->data_size);
	if (new_data == NULL) {
		printf("nosvec_resize | Failed to resize address %p!", new_data);
		return false;
	}
	vec->len = new_len;
	vec->v = new_data;
	return true;
}

#ifdef TESTS
void nosvec_test_new() {
	NOSVec vec = nosvec_new(5, sizeof(int), DATATYPE_U32);
	if (!vec.v || vec.len != 5 || vec.data_type != DATATYPE_U32) {
		printf("nosvec_test_new | test failed!\n");
		return;
	}
	printf("nosvec_test_new | test passed.\n");
}

void nosvec_test_resize() {
	NOSVec vec = nosvec_new(3, sizeof(int), DATATYPE_I32);

	bool success = nosvec_resize(&vec, 7);
	if (!success) {
		printf("nosvec_test_resize | test failed!\n");
		return;
	}
	printf("nosvec_test_resize | test passed.\n");
}

void nosvec_test() {
	nosvec_test_new();
	nosvec_test_resize();
	printf("nosvec_test | finished all tests.\n");
}
#endif
