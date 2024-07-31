#include "nosmap_all.h"


int main() {
#ifdef TESTS
	{
	printf("---------- Run tests ----------\n");
	nosvec_test();
	vasthash_test();
	nosmap_test();
	printf("---------- End of tests ----------\n");
	}
#endif
	{ // NOSVec
		// Create a vector of unsigned 32-bit integers with a length of 10
		NOSVec vec = nosvec_new(10, (DataInfo){sizeof(uint32_t), DATATYPE_U32});

		// Print the vector elements (values will be indeterminate)
		for (uint64_t i = 0; i < vec.len; i++) {
			printf("%u ", ((uint32_t *)vec.v)[i]);
		}
		printf("\n");
		printf("\n");

		// Use the vector
		for (uint64_t i = 0; i < vec.len; i++) {
			((uint32_t *)vec.v)[i] = i * i * i;
		}

		nosvec_resize(&vec, 20);

		// Print the vector elements (values will be indeterminate)
		for (uint64_t i = 0; i < vec.len; i++) {
			printf("%u ", ((uint32_t *)vec.v)[i]);
		}
		printf("\n");
	}
	{ // NOSMap
		// NOSMap map = nosmap_new();
	}
	return 0;
}
