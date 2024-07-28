#include "nosvec.c"

void test_run() {
#ifdef TESTS
	printf("---------- Run tests ----------\n");
	nosvec_test();
	printf("---------- End of tests ----------\n");
#endif
}


int main() {
	test_run();
	// Create a vector of unsigned 32-bit integers with a length of 10
	NOSVec vec = nosvec_new(10, sizeof(uint32_t), DATATYPE_U32);

	// Print the vector elements (values will be indeterminate)
	for (uint64_t i = 0; i < vec.len; i++) {
		printf("%u ", ((uint32_t *)vec.v)[i]);
	}
	printf("\n");
	printf("\n");

	// Use the vector
	for (uint64_t i = 0; i < vec.len; i++) {
		((uint32_t *)vec.v)[i] = i * 2;
	}

	nosvec_resize(&vec, 20);

	// Print the vector elements (values will be indeterminate)
	for (uint64_t i = 0; i < vec.len; i++) {
		printf("%u ", ((uint32_t *)vec.v)[i]);
	}
	printf("\n");

	return 0;
}
