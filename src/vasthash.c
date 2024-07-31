#include "vasthash.h"
#include "nosvec.h"


__m256i vasthash_impl(__m256i input_data) {
	__m256i mask = _mm256_set_epi64x(9928278621127670147u, 1954899363002243873, 2067898264094941423, 6205865627071447409);
	return _mm256_xor_si256(input_data, mask);
}

uint64_t vasthash_sum_u64x4_scalar(__m256i input_data) {
	return input_data[0] + input_data[1]+ input_data[2] + input_data[3];
}

uint64_t vasthash_hash(NOSVec *input_data) {
#if !defined(__OPTIMIZE__) || (__OPTIMIZE__ == 0)
	if (input_data->info.type != DATATYPE_M256I) {
		printf("vasthash | input_data->data_type is not DATATYPE_M256I! Returning UINT64_MAX.\n");
		return UINT64_MAX;
	}
#endif
	__m256i hash = _mm256_setzero_si256();
	for (size_t i = 0; i < input_data->len; i++) {
		hash = _mm256_add_epi64(hash, vasthash_impl((__m256i)((__m256i *)input_data->v)[i]));
	}
	return vasthash_sum_u64x4_scalar(hash);
}

uint64_t vasthash_hash_u8(NOSVec *input_data) {
#if !defined(__OPTIMIZE__) || (__OPTIMIZE__ == 0)
	if (input_data->info.type != (DATATYPE_U8 | DATATYPE_PTR)) {
		printf("vasthash | input_data->data_type is not (DATATYPE_U8 | DATATYPE_PTR)! Returning UINT64_MAX.\n");
		return UINT64_MAX;
	}
#endif
	__m256i hash = _mm256_setzero_si256();
	uint8_t *p = ((uint8_t *)input_data->v);
	uint8_t *iter = p;
	for (; iter < p + input_data->len - 32; iter += 32) {
		hash = _mm256_add_epi64(hash, vasthash_impl(*(__m256i *)p));
	}
	__m256i remaining = _mm256_setzero_si256();
	for (size_t i = 0; iter < p + input_data->len; ++iter, ++i) {
		((uint8_t *)&remaining)[i] = *iter;
	}
	hash = _mm256_add_epi64(hash, vasthash_impl(remaining));
	return vasthash_sum_u64x4_scalar(hash);
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
	NOSVec u64vec = nosvec_new(2, (DataInfo){sizeof(__m256i), DATATYPE_M256I});
	for (size_t i = 0; i < u64vec.len; i++) {
		((__m256i *)u64vec.v)[i] = _mm256_set1_epi64x(123);
	}
	uint64_t result = vasthash_hash(&u64vec);
	if (result == 3420395603173502432) {
		printf("vasthash_test_hash | test passed.\n");
		return;
	}
	printf("vasthash_test_hash | test failed!\n");
}

void vasthash_test_hash_invaild() {
	NOSVec u64vec = nosvec_new(0, (DataInfo){sizeof(uint64_t), DATATYPE_U64});
	uint64_t result = vasthash_hash(&u64vec);
	if (result == UINT64_MAX) {
		printf("vasthash_test_hash_invaild | test passed.\n");
		return;
	}
	printf("vasthash_test_hash_invaild | test failed!\n");
}

void vasthash_test_hash_u8() {
	NOSVec u8vec = nosvec_new_u8_ptr((uint8_t[5]){"apple"}, 5);
	uint64_t result = vasthash_hash_u8(&u8vec);
	if (result == 1710197409330226851) {
		printf("vasthash_test_hash_u8 | test passed.\n");
		return;
	}
	printf("vasthash_test_hash_u8 | test failed!\n");
}

void vasthash_test() {
	vasthash_test_impl();
	vasthash_test_hash();
	vasthash_test_hash_invaild();
	vasthash_test_hash_u8();
	printf("vasthash_test | finished all tests.\n");
}
#endif
