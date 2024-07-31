#include "nosmap.hpp"


template <typename Key, typename Value>
NOSMap<Key, Value>::NOSMap() : NOSMap(DEFAULT_INITIAL_CAPACITY, DEFAULT_GROW_SIZE, DEFAULT_LOAD_FACTOR) {}


template <typename Key, typename Value>
NOSMap<Key, Value>::NOSMap(size_t initialCapacity, float growSize, float loadFactor) :
	load(0), grow_size(growSize), load_factor(loadFactor) {
}

template <typename Key, typename Value>
void NOSMap<Key, Value>::test() {

}
