#include "nosmap.h"


NOSMap nosmap_new(size_t initial_capacity, size_t data_size, uint32_t data_type) {
	NOSMap map = {0, NULL};
	map.key_value = nosvec_new(initial_capacity, data_size, data_type);
	if (map.key_value.v == NULL) {
		printf("nosmap_new | Failed to allocate address %p!\n", map.key_value.v);
		return map;
	}
	map.size = initial_capacity;
	return map;
}

NOSMapSearchResult nosmap__find_bucket(NOSMap *map, NOSVec *key) {
	size_t index = vasthash_hash(key) % map->size;

	while (map->hashes_1_byte[index] & 1 != 0) {

	}
}

#ifdef TESTS
void nosmap_test() {
	printf("nosmap_test | finished all tests.\n");
}
#endif
