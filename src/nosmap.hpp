#pragma once
#include <vector>
#include <cstdint>


template <typename Key, typename Value> class NOSMap {
private:
	std::vector<uint8_t> hashes_1_byte;
	std::vector<Key, Value> key_value;
	std::vector<uint64_t>full_hashes;

	size_t load;
	float grow_size;
	float load_factor;

	static constexpr size_t DEFAULT_INITIAL_CAPACITY = 16;
	static constexpr float DEFAULT_GROW_SIZE = 1.618f;
	static constexpr float DEFAULT_LOAD_FACTOR = 0.9f;
public:
	NOSMap();
	explicit NOSMap(size_t initialCapacity = DEFAULT_INITIAL_CAPACITY, float growSize = DEFAULT_GROW_SIZE, float loadFactor = DEFAULT_LOAD_FACTOR);

	void test();
};
