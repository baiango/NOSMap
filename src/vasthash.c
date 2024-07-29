#include "vasthash.h"


__m256i vasthash_impl(__m256i input_data) {
	__m256i mask = _mm256_set_epi64x(9928278621127670147u, 1954899363002243873, 2067898264094941423, 6205865627071447409);
	return _mm256_xor_si256(input_data, mask);
}

uint64_t sum_u64x4_scalar(__m256i input_data) {
	return input_data[0] + input_data[1]+ input_data[2] + input_data[3];
}

uint64_t vasthash(NOSVec *input_data) {
	if (input_data->data_type != DATATYPE_M256I) {
		printf("vasthash | input_data->type is not DATATYPE_M256I! Returning 0.");
		return 0;
	}
	__m256i hash = _mm256_set1_epi64x(0);
	for (size_t i = 0; i < input_data->len; i++) {
		hash = _mm256_add_epi64(hash, vasthash_impl((__m256i)((__m256i *)input_data->v)[i]));
	}
	return sum_u64x4_scalar(hash);
}

#ifdef TESTS
void vasthash_test_impl() {
	__m256i input_data = _mm256_set1_epi64x(123);
	__m256i result = vasthash_impl(input_data);

	__m256i expected = _mm256_set_epi64x(9928278621127670264u, 1954899363002243930u, 2067898264094941332u, 6205865627071447306u);
	__m256i comparison = _mm256_xor_si256(result, expected);
	bool success = _mm256_testz_si256(comparison, comparison) == 1;
	if (success) {
		printf("vasthash_test_impl | test passed.\n");
		return;
	}
	printf("vasthash_test_impl | test failed!\n");
}

void vasthash_test_hash() {
	NOSVec u64vec = nosvec_new(2, sizeof(__m256i), DATATYPE_M256I);
	for (size_t i = 0; i < u64vec.len; i++) {
		((__m256i *)u64vec.v)[i] = _mm256_set1_epi64x(123);
	}
	uint64_t result = vasthash(&u64vec);
	if (result == 3420395603173502432) {
		printf("vasthash_test_hash | test passed.\n");
		return;
	}
	printf("vasthash_test_hash | test failed!\n");
}

void vasthash_test() {
	vasthash_test_impl();
	vasthash_test_hash();
	printf("vasthash_test | finished all tests.\n");
}
#endif
