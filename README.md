# ✨ NOSmap - Dinitrogen oxide Hashmap
NOSMap is a virtual homework experimental AVX2 accelerated hashmap ​project that aims to be compatible with built-in hashmap while having a 95% load factor while being fast for low-latency number crunching and memory-intensive data processing algorithms at disregard of quadratic implementation difficulty and portability. However, multiple mini unit tests are made to ensure correctness of NOSMap.

NOSMap will speed up training tokenizer by minimizing memory reads and writes, reducing memory usage and computation to the minimum possible.

NOSMap was designed for to end the users' search for the next fastest Hashmap on x86 CPU for few years. NOSMap will also ace most of the section on benchmarks from people; only if you can compile it, and the chance is 100% on AVX2 system because of the ease of the Rust installer.

# 🀄 Usage cautions
NOSMap will copy whatever you put in it, and then pack them together, thus you must insert pointers for the large struct. The design of this is due to the overhead of the pointers.
You must use nightly Rustc to compile NOSMap, or it'll fail because of Rustc didn't support portable SIMD with the stable and beta version.

# #️⃣😴😴😴 Architecture high-level overview
NOSMap was inspired by GPref's design, which is adding 2 bytes together and use it as a hash to find new buckets.

NOSMap uses way different design, it has 3 important layers of arrays.
```rs
struct NOSMap {
	1_b_hashes: Vec<u8x32>
	key_values: Vec<K, V>
	hashes: Vec<u64>
	stride_table: [u8; 256]
}
```

The key will be hashed by VastHash, then uses a fixed-sized stride table adding the probing distance to decide the next index for linear probe to find an empty bucket when keys are collided, and insert into a bucket. And it uses AVX2's `XOR` and `Ptest` for fast key comparison.

VastHash takes a `&[u64x4]` and does a bitwise XOR with a fixed constant to each `u64x4` and summing it up. The distribution quality of VastHash was better than DJB2, but excels most algorithm with prime-sized vector.

The fixed-sized stride table is designed to always hit at least 3 probes in 32 indices in `1_b_hashes` when the load is low. It's also much more resistant to clustering than linear probing, the shorter travel of it provides high spatial locality.

# 🧻🤣🤣🤣 References
[Faster than Rust and C++: the PERFECT hash table](https://youtu.be/DMQ_HcNSOAI)  
[Array of structures (AoS) and structure of arrays (SoA)](https://en.wikipedia.org/wiki/AoS_and_SoA#Structure_of_arrays)  
[Optimizing Open Addressing - thenumb.at](https://thenumb.at/Hashtables/)  
[C++Now 2018: You Can Do Better than std::unordered_map: New Improvements to Hash Table Performance](https://youtu.be/M2fKMP47slQ)  
[CppCon 2016: Timur Doumler “Want fast C++? Know your hardware!"](https://youtu.be/BP6NxVxDQIs)  
[dendibakh/perf-book](https://github.com/dendibakh/perf-book)  
