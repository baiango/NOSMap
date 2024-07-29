#include "nosvec.h"


__attribute__((destructor)) void nosvec_destroy(NOSVec* vec) {
#if !defined(__OPTIMIZE__) || (__OPTIMIZE__ == 0)
	printf("nosvec_destroy | Destroyed NOSVec #%p!\n", vec);
#endif
	free(vec->v);
}

NOSVec nosvec_new(size_t len, DataInfo data_info) {
	NOSVec vec = {NULL, 0, (DataInfo){0, DATATYPE_UNDEF}};
	vec.v = malloc(len * data_info.size);
	if (vec.v == NULL) {
		printf("nosvec_new | Failed to allocate address %p!\n", vec.v);
		return vec;
	}
	vec.len = len;
	vec.info.size = data_info.size;
	vec.info.type = data_info.type;
	return vec;
}

bool nosvec_resize(NOSVec *vec, size_t new_len) {
	void *new_data = realloc(vec->v, new_len * vec->info.size);
	if (new_data == NULL) {
		printf("nosvec_resize | Failed to resize address %p!\n", new_data);
		return false;
	}
	vec->v = new_data;
	vec->len = new_len;
	return true;
}

#ifdef TESTS
void nosvec_test_new() {
	NOSVec vec = nosvec_new(5, (DataInfo){sizeof(int32_t), DATATYPE_U32});
	if (vec.v && vec.len == 5 && vec.info.type == DATATYPE_U32) {
		printf("nosvec_test_new | test passed.\n");
		return;
	}
	printf("nosvec_test_new | test failed!\n");
}

void nosvec_test_resize() {
	NOSVec vec = nosvec_new(3, (DataInfo){sizeof(uint32_t), DATATYPE_I32});
	nosvec_resize(&vec, 7);

	bool success = vec.len == 7;
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
