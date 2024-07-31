#include "nosmap.h"


NOSMap nosmap_new(NOSMapNewParameters *params) {
	bool exit = false;
	if (params->key_info.size == 0) {
		printf("nosmap_new | key_info.size %u | Please set params->key_info.size\n", params->key_info.size);
		exit = true;
	}
	if (params->key_info.type == DATATYPE_UNDEF) {
		printf("nosmap_new | key_info.type %u | Please set params->key_info.type\n", params->key_info.type);
		exit = true;
	}
	if (params->value_info.size == 0) {
		printf("nosmap_new | value_info.size %u | Please set params->value_info.size\n", params->value_info.size);
		exit = true;
	}
	if (params->value_info.type == DATATYPE_UNDEF) {
		printf("nosmap_new | value_info.type %u | Please set params->value_info.type\n", params->value_info.type);
		exit = true;
	}
	if (params->grow_size <= 0.0) {
		printf("nosmap_new | grow_size %f | Please set params->grow_size higher than 0.0\n", params->grow_size);
		exit = true;
	}
	if (params->load_factor <= 0.0 || params->load_factor >= 1.0) {
		printf("nosmap_new | load_factor %f | Please set params->load_factor between 0.0 to 1.0\n", params->load_factor);
		exit = true;
	}
	NOSMap map = {
		(DataInfo){0, DATATYPE_UNDEF},
		(DataInfo){0, DATATYPE_UNDEF},
		NULL, NULL,
		0, 0.0, 0.0
	};
	if (exit) {
		return map;
	}

	map.hashes_1_byte = malloc(params->capacity * sizeof(uint8_t));
	if (map.hashes_1_byte == NULL) {
		printf("nosmap_new | Failed to allocate hashes_1_byte #%p!\n", map.hashes_1_byte);
		return map;
	}
	map.full_hashes = malloc(params->capacity * sizeof(uint8_t));
	if (map.full_hashes == NULL) {
		printf("nosmap_new | Failed to allocate full_hashes #%p!\n", map.hashes_1_byte);
		return map;
	}
	map.key_value = nosvec_new(params->capacity, (DataInfo){sizeof(NOSMapKeyValue), DATATYPE_NOSMAP_KEY_VALUE});
	if (map.key_value.v == NULL) {
		printf("nosmap_new | Failed to allocate address #%p!\n", map.key_value.v);
		return map;
	}

	map.key_info = params->key_info;
	map.value_info = params->value_info;
	map.load = 0;
	map.grow_size = params->grow_size;
	return map;
}

NOSMapNewParameters nosmap_default_params() {
	return (NOSMapNewParameters){
		(DataInfo){0, DATATYPE_UNDEF},
		(DataInfo){0, DATATYPE_UNDEF},
		16,
		1.618,
		0.9
	};
}

NOSMapSearchResult nosmap__find_bucket(NOSMap *map, NOSVec *key) {
#if !defined(__OPTIMIZE__) || (__OPTIMIZE__ == 0)
	if (key->info.type != (DATATYPE_U8 | DATATYPE_PTR)) {
		printf("nosmap__find_bucket | key->info.type %8x | key->info.type is not (DATATYPE_U8 | DATATYPE_PTR)! Returning (NOSMapSearchResult){UINT32_MAX, UINT64_MAX, false}.\n", key->info.type);
		return (NOSMapSearchResult){UINT32_MAX, UINT64_MAX, false};
	}
	if (((uint8_t *)key->v)[key->len] == 0) {
		printf("nosmap__find_bucket | key->v might be null-terminated. May SIGSEGV incoming!\n");
	}
#endif
	uint64_t hash = vasthash_hash_u8(key);
	uint64_t index = hash % map->key_value.len;
	while ((map->hashes_1_byte[index] & (NOSMAP_OCCUPIED | NOSMAP_TOMESTONE)) != NOSMAP_EMPTY) {
		uint8_t *key_data = (uint8_t *)key->v;
		if (map->hashes_1_byte[index] == key_data[index]) {
			return (NOSMapSearchResult){index, true};
		}

		uint64_t next_stride = (key_data[0] + key_data[key->len - 1]) * 2 + 1;
		index = (index + next_stride) % map->key_value.len;
	}
	return (NOSMapSearchResult){index, hash, false};
}

void nosmap_put(NOSMap *map, NOSVec key, NOSVec value) {
	NOSMapSearchResult search_result = nosmap__find_bucket(map, &key);
	((NOSMapKeyValue *)map->key_value.v)[search_result.index] = (NOSMapKeyValue){key, value};
	map->hashes_1_byte[search_result.index] = search_result.hash & 0xff;
	map->load += 1;

	if (map->load > map->key_value.len * map->load_factor) {
		size_t new_capacity = (map->key_value.len * map->grow_size);
		nosmap__resize(map, new_capacity);
	}
}

NOSVec *nosmap_get(NOSMap *map, NOSVec *key) {
	NOSMapSearchResult search_result = nosmap__find_bucket(map, key);
	if (search_result.found) {
		return &((NOSMapKeyValue *)map->key_value.v)[search_result.index].value;
	}
	return NULL;
}

void nosmap_remove(NOSMap *map, NOSVec *key) {
	NOSMapSearchResult search_result = nosmap__find_bucket(map, key);
	if (search_result.found) {
		map->hashes_1_byte[search_result.index] &= ~(NOSMAP_OCCUPIED | NOSMAP_TOMESTONE);
	}
}

void nosmap__resize(NOSMap *map, size_t new_capacity) {
	printf("HelloHelloHelloHelloHello\n");

	NOSVec *old_buckets = &map->key_value;
	uint8_t *old_hashes_1_byte = map->hashes_1_byte;
	map->load = 0;

	NOSVec new_buckets = nosvec_new(new_capacity, (DataInfo){sizeof(NOSMapKeyValue), DATATYPE_NOSMAP_KEY_VALUE});
	uint8_t *new_hashes_1_byte = calloc(new_capacity, sizeof(uint8_t));
	if (new_hashes_1_byte == NULL) {
		printf("nosmap__resize | Failed to allocate address %p!\n", new_hashes_1_byte);
		return;
	}
	uint64_t *new_full_hashes = calloc(new_capacity, sizeof(uint64_t));
	if (new_full_hashes == NULL) {
		printf("nosmap__resize | Failed to allocate address %p!\n", new_full_hashes);
		return;
	}

	for (size_t i = 0; i < old_buckets->len; ++i) {
		if ((old_hashes_1_byte[i] & (NOSMAP_OCCUPIED | NOSMAP_TOMESTONE)) != NOSMAP_EMPTY) {
			NOSMapKeyValue key_value = ((NOSMapKeyValue *)old_buckets->v)[i];

			NOSMapSearchResult search_result = nosmap__find_bucket(map, &key_value.key);
			new_hashes_1_byte[search_result.index] = search_result.hash % new_buckets.len & ~(NOSMAP_OCCUPIED | NOSMAP_TOMESTONE) | NOSMAP_OCCUPIED;
			((NOSMapKeyValue *)new_buckets.v)[search_result.index] = key_value;
			new_full_hashes[search_result.index] = search_result.hash;
			map->load += 1;
		}
	}

	// Free old memory
	free(old_buckets->v);
	free(old_hashes_1_byte);

	map->key_value = new_buckets;
	map->hashes_1_byte = new_hashes_1_byte;
}

#ifdef TESTS
void nosmap_test_new() {
	NOSMapNewParameters params = nosmap_default_params();
	params.key_info = (DataInfo){sizeof(uint8_t *), DATATYPE_U8 | DATATYPE_PTR};
	params.value_info = (DataInfo){sizeof(uint32_t), DATATYPE_U32};
	params.capacity = 7;
	NOSMap map = nosmap_new(&params);

	bool success = map.key_info.size == sizeof(uint8_t *)
		&& map.key_info.type == (DATATYPE_U8 | DATATYPE_PTR)
		&& map.value_info.size == sizeof(uint32_t)
		&& map.value_info.type == DATATYPE_U32
		&& map.key_value.len == 7;
	if (success) {
		printf("nosmap_test_new | test passed.\n");
		return;
	}
	printf("nosmap_test_new | test failed!\n");
}

void nosmap_test_resize() {
	NOSMapNewParameters params = nosmap_default_params();
	params.key_info = (DataInfo){sizeof(uint8_t *), DATATYPE_U8 | DATATYPE_PTR};
	params.value_info = (DataInfo){sizeof(uint32_t), DATATYPE_U32};
	params.capacity = 2;
	NOSMap map = nosmap_new(&params);
	nosmap__resize(&map, 16);
	if (map.key_value.len == 16) {
		printf("nosmap_test_resize | test passed.\n");
		return;
	}
	printf("nosmap_test_resize | test failed!\n");
}

void nosmap_test_general() {
	NOSMapNewParameters params = nosmap_default_params();
	params.key_info = (DataInfo){sizeof(uint8_t *), DATATYPE_U8 | DATATYPE_PTR};
	params.value_info = (DataInfo){sizeof(uint32_t), DATATYPE_U32};
	params.capacity = 2;
	NOSMap map = nosmap_new(&params);

	NOSVec key = nosvec_new_u8_ptr((uint8_t[5]){"apple"}, 5);
	NOSVec value = nosvec_new(3, (DataInfo){sizeof(uint32_t), DATATYPE_U32});
	((uint32_t *)value.v)[0] = 2345;
	if (((uint8_t *)key.v)[0] != 0x61) {
		printf("nosmap_test_general | key.v %2x | key.v is not %2x.", ((uint8_t *)key.v)[0], 0x61);
	}
	if (key.info.type != (DATATYPE_U8 | DATATYPE_PTR)) {
		printf("nosmap_test_general | key.info.type %4x | key.info.type is not %4x (DATATYPE_U8 | DATATYPE_PTR)\n", key.info.type, DATATYPE_U8 | DATATYPE_PTR);
	}
	if (((uint32_t *)value.v)[0] != 2345) {
		printf("nosmap_test_general | key.v %4x | key.v is not %4x.", ((uint32_t *)value.v)[0], 2345);
	}
	if (value.info.type != DATATYPE_U32) {
		printf("nosmap_test_general | value.info.type %4x | value.info.type is not %4x DATATYPE_U32\n", value.info.type, DATATYPE_U32);
	}
	nosmap_put(&map, key, value);

	bool success = true;
	if (success) {
		printf("nosmap_test_general | test passed.\n");
		return;
	}
	printf("nosmap_test_general | test failed!\n");
}

void nosmap_test() {
	nosmap_test_new();
	nosmap_test_resize();
	nosmap_test_general();
	printf("nosmap_test | finished all tests.\n");
}
#endif
