#include "nosvec.h"


__attribute__((destructor))
void nosvec_destroy() {}

NOSVec nosvec_new(size_t len, DataInfo data_info) {
	NOSVec vec = {NULL, 0, (DataInfo){0, DATATYPE_UNDEF}};
	vec.v = malloc(len * data_info.size);
	if (vec.v == NULL) {
		printf("nosvec_new | Failed to allocate address %p!\n", vec.v);
		return vec;
	}
	vec.len = len;
	vec.info = data_info;
	return vec;
}

NOSVec nosvec_new_u8_ptr(uint8_t *bytes, size_t len) {
	return (NOSVec){bytes, len, (DataInfo){sizeof(uint8_t *), DATATYPE_U8 | DATATYPE_PTR}};
}

bool nosvec_resize(NOSVec *vec, size_t new_len) {
	vec->v = realloc(vec->v, new_len * vec->info.size);
	if (vec->v == NULL) {
		printf("nosvec_resize | Failed to resize address %p!\n", vec->v);
		return false;
	}
	vec->len = new_len;
	return true;
}

#ifdef TESTS
void nosvec_test_new() {
	NOSVec vec = nosvec_new(5, (DataInfo){sizeof(int32_t), DATATYPE_I32});
	if (vec.v && vec.len == 5 && vec.info.type == DATATYPE_I32) {
		printf("nosvec_test_new | test passed.\n");
		return;
	}
	printf("nosvec_test_new | test failed!\n");
}

void nosvec_test_new_ptr() {
	NOSVec vec = nosvec_new(1, (DataInfo){sizeof(int32_t *), DATATYPE_I32 | DATATYPE_PTR});
	if (vec.v && vec.len == 1 && vec.info.type == (DATATYPE_I32 | (1 << 30))) {
		printf("nosvec_test_new_ptr | test passed.\n");
		return;
	}
	printf("nosvec_test_new_ptr | test failed!\n");
}

void nosvec_test_resize() {
	NOSVec vec = nosvec_new(3, (DataInfo){sizeof(uint32_t), DATATYPE_U32});
	((uint32_t *)vec.v)[2] = 1234567;
	nosvec_resize(&vec, 7);

	bool success = vec.len == 7
		&& ((uint32_t *)vec.v)[2] == 1234567;
	if (success) {
		printf("nosvec_test_resize | test passed.\n");
		return;
	}
	printf("nosvec_test_resize | test failed!\n");
}

void nosvec_test() {
	nosvec_test_new();
	nosvec_test_new_ptr();
	nosvec_test_resize();
	printf("nosvec_test | finished all tests.\n");
}
#endif
