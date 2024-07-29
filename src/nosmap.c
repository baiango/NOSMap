#include "nosmap.h"
#include "nosvec.h"


NOSMap nosmap_new(size_t initial_capacity, DataInfo key_info, DataInfo value_info) {
	NOSMap map = {
		0,
		(DataInfo){0, DATATYPE_UNDEF},
		(DataInfo){0, DATATYPE_UNDEF},
		NULL, NULL
	};
	map.hashes_1_byte = malloc(initial_capacity * sizeof(uint8_t));
	if (map.hashes_1_byte == NULL) {
		printf("nosmap_new | Failed to allocate address %p!\n", map.hashes_1_byte);
		return map;
	}
	map.key_value = nosvec_new(initial_capacity, (DataInfo){sizeof(NOSMapKeyValue), DATATYPE_NOSMAP_KEY_VALUE});
	if (map.key_value.v == NULL) {
		printf("nosmap_new | Failed to allocate address %p!\n", map.key_value.v);
		return map;
	}
	map.key_info.size = key_info.size;
	map.key_info.type = key_info.type;
	map.value_info.size = value_info.size;
	map.value_info.type = value_info.type;
	map.size = initial_capacity;
	return map;
}

NOSMapSearchResult nosmap__find_bucket(NOSMap *map, NOSVec *key) {
#if !defined(__OPTIMIZE__) || (__OPTIMIZE__ == 0)
	if (key->info.type != DATATYPE_U8) {
		printf("vasthash | input_data->data_type is not DATATYPE_U8! Returning {UINT64_MAX, false}.\n");
		return (NOSMapSearchResult){UINT64_MAX, false};
	}
#endif
	uint32_t index = vasthash_hash(key) % map->size;
	const uint32_t EMPTY = 0;
	while ((map->hashes_1_byte[index] & 1) != EMPTY) {
		uint8_t *key_data = (uint8_t *)key->v;
		if (map->hashes_1_byte[index] == key_data[index]) {
			return (NOSMapSearchResult){index, true};
		}

		uint32_t next_stride = (key_data[0] + key_data[key->len - 1]) * 2 + 1;
		index = (index + next_stride) % map->key_value.len;
	}
	return (NOSMapSearchResult){index, false};
}

#ifdef TESTS
void nosmap_test_new() {
	NOSMap map = nosmap_new(
		7,
		(DataInfo){sizeof(uint8_t *), DATATYPE_U8 | DATATYPE_PTR},
		(DataInfo){sizeof(uint32_t), DATATYPE_U32}
	);

	bool success = map.key_info.size == sizeof(uint8_t *)
		&& map.key_info.type == DATATYPE_U8 | DATATYPE_PTR
		&& map.value_info.size == sizeof(uint32_t)
		&& map.value_info.type == DATATYPE_U32
		&& map.key_value.len == 7;
	if (success) {
		printf("nosmap_test_new | test passed.\n");
		return;
	}
	printf("nosmap_test_new | test failed!\n");
}

void nosmap_test() {
	nosmap_test_new();
	printf("nosmap_test | finished all tests.\n");
}
#endif
