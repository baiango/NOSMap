# ✨ NOSmap - Dinitrogen oxide Hashmap
NOSMap is a virtual homework experimental AVX2 accelerated hashmap ​project that aims to have a 90% load factor while being fast for low-latency number crunching and memory-intensive data processing algorithms at disregard of quadratic implementation difficulty and portability. However, multiple mini unit tests are made to ensure correctness of NOSMap.

NOSMap will speed up training tokenizer by minimizing memory reads and writes, reducing memory usage and computation to the minimum possible.

NOSMap was designed for to end the users' search for the next fastest Hashmap on x86 CPU for few years.

NOSMap will also ace most of the section on benchmarks from people; only if you can compile it, and the chance is 100.0% on AVX2 system because of the ease of the [UCRT64 GCC](https://www.msys2.org/) installer, and GCC is bundled in every Linux installation.

# 🎇🎆 Gain
- High correctness and reliability - The features are highly tested
- Possibly read-Only Concurrent Hash Map with Write Lock
- Few macros only

# 🚤🔥 Drawbacks - Reason
- Written in C instead of Rust - My skill issues 😭😭😭 ("I write assembly code in C that GCC can't even generate on its own.")
- Abuses uninitialized memory - To speed up NOSMap
- Use GCC extensions to optimize the code - My skill issues and to improve readability (It can be compiled into static library to enlarge the portiblity)
- Need a decompiler to read the code 🗿🙄👽

# 🧻🤣🤣🤣 References
[Faster than Rust and C++: the PERFECT hash table](https://youtu.be/DMQ_HcNSOAI)  
[Array of structures (AoS) and structure of arrays (SoA)](https://en.wikipedia.org/wiki/AoS_and_SoA#Structure_of_arrays)  
[Optimizing Open Addressing - thenumb.at](https://thenumb.at/Hashtables/)  
[C++Now 2018: You Can Do Better than std::unordered_map: New Improvements to Hash Table Performance](https://youtu.be/M2fKMP47slQ)  
[CppCon 2016: Timur Doumler “Want fast C++? Know your hardware!"](https://youtu.be/BP6NxVxDQIs)  
[dendibakh/perf-book](https://github.com/dendibakh/perf-book)  

# #️⃣😴😴😴 Architecture high-level overview
NOSMap was inspired by GPref's design, which is adding 2 bytes together and use it as a hash to find new buckets.

NOSMap uses way different design, it has 3 important layers of arrays.
```rs
struct NOSMap {
	1_b_hashes: Vec<u8x32>
	key_values: Vec<K, V>
	hashes: Vec<u64>
}
```

The key will be hashed by VastHash, then uses the first and the last byte of the key as dynamic hashing to decide the next index for linear probe to find an empty bucket when keys are collided, and insert into a bucket. And it uses AVX2's `XOR` and `PTest` for fast key comparison.

VastHash takes a `&[u64x4]` and does a bitwise XOR with a fixed constant to each `u64x4` and summing it up. The distribution quality of VastHash was better than DJB2, but excels most algorithm with prime-sized vector.

The dynamic hashing is designed to be resistant to clustering than linear probing, and the shorter travel of it provides higher spatial locality than double hashing.

# 🧁🎈 A simplified architecture of NOSMap
```py
class NOSMap:
	def __init__(self, initial_capacity=16):
		assert initial_capacity > 1
		self.capacity = initial_capacity
		self.size = 0
		self.buckets = [None] * self.capacity

	def _find_bucket(self, key):
		index = hash(key) % self.capacity

		while self.buckets[index] != None:
			if self.buckets[index][0] == key:
				return index, True

			next_stride = (ord(key[0]) + ord(key[-1])) * 2 + 1
			index = (index + next_stride) % self.capacity

		return index, False

	def put(self, key, value):
		index, _ = self._find_bucket(key)
		self.buckets[index] = (key, value)
		self.size += 1
		if self.size > self.capacity * 0.90:
			self._resize()

	def get(self, key):
		index, found = self._find_bucket(key)
		return self.buckets[index][1] if found else None

	def remove(self, key):
		index, found = self._find_bucket(key)
		if found:
			self.buckets[index] = (None, None)

	def _resize(self):
		old_buckets = self.buckets
		self.capacity *= 2
		self.size = 0
		self.buckets = [None] * self.capacity
		for bucket in old_buckets:
			if bucket in [None, (None, None)]:
				continue
			self.put(*bucket)

	def __str__(self):
		items = []
		for i, bucket in enumerate(self.buckets):
			if bucket in [None, (None, None)]:
				continue
			items.append(f"{bucket[0]}[{i}]: {bucket[1]}")
		return "{" + ", ".join(items) + "}"

# Example usage:
hash_map = NOSMap(initial_capacity=2)
hash_map.put("apple", 2)
hash_map.put("apple", 1)
hash_map.put("grape", 1)
hash_map.put("nuts", 4)
hash_map.put("banana", 2)
print("hash_map", hash_map) # {nuts[3]: 4, grape[5]: 1, banana[6]: 2, apple[11]: 1}
print('hash_map.get("apple")', hash_map.get("apple"))  # Output: 1
print('hash_map.get("banana")', hash_map.get("banana"))  # Output: 2
hash_map.remove("apple")
hash_map.remove("aaa")
print('hash_map.get("apple")', hash_map.get("apple"))  # Output: None
print("hash_map",hash_map)  # Output: {nuts[3]: 4, grape[5]: 1, banana[6]: 2, None[11]: None}
```