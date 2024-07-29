#pragma once
#include <immintrin.h>
#include <stdint.h>
#include <stdbool.h>
#include <stdio.h>


__m256i vast_hash_impl(__m256i input_data);
uint64_t sum_u64x4_scalar(__m256i input_data);
#ifdef TESTS
void vast_hash_test_impl();
void vast_hash_test();
#endif
