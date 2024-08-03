# 🤗✨ NOSmap - Dinitrogen oxide Hashmap
NOSMap is a virtual homework experimental AVX2 accelerated hashmap ​project that aims to have a 97% load factor while being fast for low-latency number crunching and memory-intensive data processing algorithms at disregard of quadratic implementation difficulty and portability. However, multiple mini unit tests are made to ensure correctness of NOSMap.

NOSMap will speed up training tokenizer by minimizing memory reads and writes, reducing memory usage and computation to the minimum possible.

NOSMap was designed for to end the users' search for the next fastest Hashmap on x86 CPU for few years.

NOSMap will also ace most of the section on benchmarks from people; only if you can compile it, and the chance is 100.0% on AVX2 system because of the ease of the Rust installation.

# 🫠🌪️🏳️ Performance
I am completely devastated by Rust hash map performance. My NOSMap's design could not beat the Rust hash map when preallocated both hash map. I will declare defeat. I am done with this hash map.

| Map Type | Key Size | Capacity | Preallocated Time | Resizing Time |
|---|---|---|---|---|
| NOSMap | 1,000,000 | 1,031,992 | 293.022ms | 380.7158ms |
| NOSMap | 10,000,000 | 10,319,918 | 4.222s | 4.951s |
| NOSMap | 80,000,000 | 82,559,344 | 39.605s | 45.892s |
| NOSMap | [303,872](https://weakpass.com/wordlist/1859) | 313,594 | 99.2489ms | 156.946ms |
| NOSMap | [38,647,798](https://weakpass.com/wordlist/1256) | 39,884,212 | 17.715s | 55.975s |
||
| HashMap | 1,000,000 | 1,144,165 | 244.8ms | 420.4349ms |
| HashMap | 10,000,000 | 11,441,647 | 3.124s | 4.749s |
| HashMap | 80,000,000 | 91,533,176 | 30.551s | 61.099s |
| HashMap | [303,872](https://weakpass.com/wordlist/1859) | 347,680 | 79.388ms | 104.899ms |
| HashMap | [38,647,798](https://weakpass.com/wordlist/1256) | 44,219,452 | 17.380s | 27.292s |

**Running command:**
```
cargo r --release
```

**Benchmark output:**
```
---------- Loading file ----------
---------- Preallocated ----------
Time elapsed for NOSMap is: 293.0227ms | key size 1000000 | capacity 1031992
Time elapsed for HashMap is: 244.8ms | key size 1000000 | capacity 1144165
Time elapsed for NOSMap is: 4.2228494s | key size 10000000 | capacity 10319918
Time elapsed for HashMap is: 3.1242706s | key size 10000000 | capacity 11441647
Time elapsed for NOSMap is: 39.6058656s | key size 80000000 | capacity 82559344
Time elapsed for HashMap is: 30.5517177s | key size 80000000 | capacity 91533176
Time elapsed for NOSMap is: 99.2489ms | key size 303872 | capacity 313594
Time elapsed for HashMap is: 79.388ms | key size 303872 | capacity 347680
Time elapsed for NOSMap is: 17.7158133s | key size 38647798 | capacity 39884212
Time elapsed for HashMap is: 17.38028s | key size 38647798 | capacity 44219452
---------- Resizing ----------
Time elapsed for NOSMap is: 380.7158ms | key size 1000000 | capacity 0
Time elapsed for HashMap is: 420.4349ms | key size 1000000 | capacity 0
Time elapsed for NOSMap is: 4.9514057s | key size 10000000 | capacity 0
Time elapsed for HashMap is: 4.7491988s | key size 10000000 | capacity 0
Time elapsed for NOSMap is: 45.8922994s | key size 80000000 | capacity 0
Time elapsed for HashMap is: 61.0990443s | key size 80000000 | capacity 0
Time elapsed for NOSMap is: 156.946ms | key size 303872 | capacity 0
Time elapsed for HashMap is: 104.8991ms | key size 303872 | capacity 0
Time elapsed for NOSMap is: 55.975341s | key size 38647798 | capacity 0
Time elapsed for HashMap is: 27.2924489s | key size 38647798 | capacity 0
```
**NOSMap setting**
```rs
	pub fn new(initial_capacity: usize) -> Self {
		let initial_prime_capacity = next_prime(initial_capacity as u32) as usize;
		let one_byte_hashes = vec![0; initial_prime_capacity];
		let key_values = vec![KeyValue::default(); initial_prime_capacity];
		let resize_hashes = vec![0; initial_prime_capacity];

		Self {
			one_byte_hashes,
			key_values,
			resize_hashes,
			load: 0,
			grow_size: 5.05,
			load_factor: 0.97,
			modulo_const: uint_div_const(initial_prime_capacity as u64) as usize
		}
	}
```
**Benchmark code**
```rs
#![feature(portable_simd)]
use std::{time::Instant, collections::HashMap, io::{BufReader, BufRead}, fs::File};
mod nosmap;
mod vasthash_b;
mod is_prime;
use nosmap::NOSMap;


fn load_file_as_vec_vec_u8(file_path: &str) -> std::io::Result<Vec<Vec<u8>>> {
	let file = File::open(file_path)?;
	let reader = BufReader::new(file);

	let mut vec_vec_u8 = Vec::new();

	for (i, line) in reader.lines().enumerate() {
		let line = line?;
		vec_vec_u8.push(line.into_bytes());
	}

	Ok(vec_vec_u8)
}

fn benchmark_1(test_size: usize, test_capacity: usize) {
	let mut keys = Vec::with_capacity(test_size);
	for i in 0..test_size {
		keys.push(Vec::<u8>::from(format!("key{}", i)));
	}
	benchmark_2(keys, test_capacity);
}

fn benchmark_2(keys: Vec<Vec<u8>>, test_capacity: usize) {
	{
		let start = Instant::now();

		let capacity = (test_capacity as f32 / 0.969).ceil() as usize;
		let mut map = NOSMap::<i32>::new(capacity);
		for (i, key) in keys.clone().into_iter().enumerate() {
			map.put(key.clone(), i as i32);
			assert_eq!(map.get(&key), Some(i as i32));
		}

		println!("Time elapsed for NOSMap is: {:?} | key size {} | capacity {}", start.elapsed(),keys.len(), capacity);
	}
	{
		let start = Instant::now();

		let capacity = (test_capacity as f32 / 0.874).ceil() as usize;
		let mut map = HashMap::with_capacity(capacity);
		for (i, key) in keys.clone().into_iter().enumerate() {
			map.insert(key.clone(), i as i32);
			assert_eq!(map.get(&key), Some(&(i as i32)));
		}

		println!("Time elapsed for HashMap is: {:?} | key size {} | capacity {}", start.elapsed(),keys.len(), capacity);
	}
}

fn main() {
	println!("---------- Loading file ----------");
	let keys_304k = load_file_as_vec_vec_u8("Top304Thousand-probable-v2.txt").unwrap();
	let keys_38m = load_file_as_vec_vec_u8("hk_hlm_founds.txt").unwrap();
	println!("---------- Preallocated ----------");
	benchmark_1(1_000_000, 1_000_000);
	benchmark_1(1_000_000_0, 1_000_000_0);
	benchmark_1(8_000_000_0, 8_000_000_0);
	benchmark_2(keys_304k.clone(), keys_304k.len());
	benchmark_2(keys_38m.clone(), keys_38m.len());
	println!("---------- Resizing ----------");
	benchmark_1(1_000_000, 0);
	benchmark_1(1_000_000_0, 0);
	benchmark_1(8_000_000_0, 0);
	benchmark_2(keys_304k.clone(), 0);
	benchmark_2(keys_38m.clone(), 0);
}
```

# 🎇🎆 Gain
- High correctness and reliability - NOSMap's features are minimal and battle-tested
- Possibly read-only concurrent hash map with write lock
- NOSMap will only slow down by bucket collisions when the load factor is 95% above. Please refrain from setting the load factor below 95%
- NOSMap will drain [hash flooding attacks](https://en.wikipedia.org/wiki/Collision_attack)

# 🚤🔥 Drawbacks - Reason
- Written in Rust instead of C - My skill issues 😭😭😭 ("I cannot fix `void *` from SIGSEGV in C.")
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
pub struct NOSMap<V> {
	pub one_byte_hashes: Vec<u8>,
	pub key_values: Vec<KeyValue<V>>,
	pub resize_hashes: Vec<u64>,

	pub load: usize,
	pub grow_size: f32,
	pub load_factor: f32,
	modulo_const: usize
}
```

The key will be hashed by VastHash-b, then uses the first byte of the byte as dynamic hashing to decide the next index for linear probe to find an empty bucket when keys are collided, and insert into a bucket.

VastHash-b takes a `&[u64x4]` and summing it up. The distribution quality of VastHash-b was better than DJB2, but excels most algorithm with prime-sized vector.

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
		next_stride = (ord(key[0]) + ord(key[-1])) * 2 + 1

		while self.buckets[index] != None:
			if self.buckets[index][0] == key:
				return index, True

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
