#include "vasthash.h"


__m256i vast_hash_impl(__m256i input_data) {
	__m256i mask = _mm256_set_epi64x(9928278621127670147u, 1954899363002243873, 2067898264094941423, 6205865627071447409);
	return _mm256_xor_si256(input_data, mask);
}


uint64_t sum_u64x4_scalar(__m256i input_data) {
	return input_data[0] + input_data[1]+ input_data[2] + input_data[3];
}

// uint64_t vast_hash(uint64_t *input_data, size_t len) {
// 	__m256i hash = _mm256_set1_epi64x(0);
// 	for (size_t i = 0; i < len; i++) {
// 		__m256i input_vec = _mm256_loadu_si256((__m256i*)&input_data[i*4]);
// 		hash = _mm256_add_epi64(hash, vast_hash_impl(input_vec));
// 	}
// 	return sum_u64x4_scalar(hash);
// }

#ifdef TESTS
void vast_hash_test_impl() {
	__m256i input_data = _mm256_set1_epi64x(123);
	__m256i result = vast_hash_impl(input_data);
	__m256i expected = _mm256_set_epi64x(9928278621127670264u, 1954899363002243930u, 2067898264094941332u, 6205865627071447306u);
	bool success = _mm256_testc_si256(result, expected) == 1;
	if (!success) {
		printf("vast_hash_test_impl | test failed!\n");
		return;
	}
	printf("vast_hash_test_impl | test passed.\n");
}

void vast_hash_test() {
	vast_hash_test_impl();
	printf("vast_hash_test | finished all tests.\n");
}
#endif
