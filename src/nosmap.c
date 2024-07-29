#include "vasthash.h"
#include "nosvec.h"
#include "nosmap.h"


NOSMap nosmap_new(uint64_t initial_capacity, uint32_t data_size, uint32_t data_type) {
	NOSMap map = {0, NULL};
	map.buckets = nosvec_new(initial_capacity, data_size, data_type);
	if (map.buckets.v == NULL) {
		printf("nosmap_new | Failed to allocate address %p!", map.buckets.v);
		return map;
	}
	map.size = initial_capacity;
	return map;
}

// NOSMapSearchResult nosmap__find_bucket(NOSMap *map, uint8_t *key) {
// 	uint64_t index = vast_hash(key) % map->size;
// }

#ifdef TESTS
void nosmap_test() {
	printf("nosmap_test | finished all tests.\n");
}
#endif
