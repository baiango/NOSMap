#include "vector.c"

void test_run() {
#ifdef TESTS
	printf("---------- Run tests ----------\n");
	test_vector();
	printf("---------- End of tests ----------\n");
#endif
}


int main() {
	test_run();
	// Create a vector of unsigned 32-bit integers with a length of 10
	Vector vec = vector_new(10, sizeof(uint32_t), TYPE_U32);

	// Print the vector elements (values will be indeterminate)
	for (uint64_t i = 0; i < vec.len; i++) {
		printf("%u ", uint32_t *(vec.v)[i]);
	}
	printf("\n");
	printf("\n");

	// Use the vector
	for (uint64_t i = 0; i < vec.len; i++) {
		((uint32_t *)vec.v)[i] = i * 2;
	}

	vector_resize(&vec, 20);

	// Print the vector elements (values will be indeterminate)
	for (uint64_t i = 0; i < vec.len; i++) {
		printf("%u ", uint32_t *(vec.v)[i]);
	}
	printf("\n");

	// Free the memory allocated for the vector
	free(vec.v);

	return 0;
}
