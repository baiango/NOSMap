#pragma once
#include <immintrin.h>
#include <stdint.h>
#include <stdbool.h>
#include <stdio.h>
#include "nosvec.h"


__m256i vasthash_impl(__m256i input_data);
uint64_t sum_u64x4_scalar(__m256i input_data);
uint64_t vasthash(NOSVec *input_data);

#ifdef TESTS
void vasthash_test_impl();
void vasthash_test_hash();
void vasthash_test();
#endif
