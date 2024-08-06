# 🤗✨ NOSmap - Dinitrogen oxide Hashmap
NOSMap is a virtual homework experimental AVX2 accelerated hashmap ​project that aims to have a 99.9% load factor while being fast for low-latency number crunching and memory-intensive data processing algorithms at disregard of quadratic implementation difficulty and portability. However, multiple mini unit tests are made to ensure correctness of NOSMap.

NOSMap will speed up training tokenizer by minimizing memory reads and writes, reducing memory usage and computation to the minimum possible.

NOSMap was designed for to end the users' search for the next fastest Hashmap on x86 CPU for few years.

NOSMap will also ace most of the section on benchmarks from people; only if you can compile it, and the chance is 100.0% on AVX2 system because of the ease of the Rust installation.

# 🫠🌪️🏳️ Performance
I am completely devastated by Rust hash map performance. My NOSMap's design could not beat the Rust hash map when preallocated both hash map. I will declare defeat. I am done with this hash map.

| Scenario | Map Type | Key Size | Missing Size (Percentage) | Initial Capacity | Time Elapsed (ms/s) |
|---|---|---|---|---|---|
| **Preallocated w/o missing** | NOSMap | 1,000,000 | 0 (inf%) | 1,001,102 (99.89%) | 640.7332 ms |
|  | HashMap | 1,000,000 | 0 (inf%) | 1,144,165 (87.40%) | 399.6438 ms |
|  | NOSMap | 10,000,000 | 0 (inf%) | 10,011,012 (99.89%) | 6.9721599 s |
|  | HashMap | 10,000,000 | 0 (inf%) | 11,441,647 (87.40%) | 5.5180007 s |
|  | NOSMap | 303,872 | 0 (inf%) | 304,207 (99.89%) | 130.4601 ms |
|  | HashMap | 303,872 | 0 (inf%) | 347,680 (87.40%) | 114.8022 ms |
| **Preallocated w/ missing** | NOSMap | 1,000,000 | 125,000 (8.00%) | 1,001,102 (99.89%) | 702.0693 ms |
|  | HashMap | 1,000,000 | 125,000 (8.00%) | 1,144,165 (87.40%) | 434.22 ms |
|  | NOSMap | 10,000,000 | 1,250,000 (8.00%) | 10,011,012 (99.89%) | 7.2867382 s |
|  | HashMap | 10,000,000 | 1,250,000 (8.00%) | 11,441,647 (87.40%) | 5.7745234 s |
|  | NOSMap | 303,872 | 37,984 (8.00%) | 304,207 (99.89%) | 2.7460905 s |
|  | HashMap | 303,872 | 37,984 (8.00%) | 347,680 (87.40%) | 120.1809 ms |
| **Resizing w/o missing** | NOSMap | 1,000,000 | 0 (inf%) | 0 (inf%) | 445.2367 ms |
|  | HashMap | 1,000,000 | 0 (inf%) | 0 (inf%) | 540.8554 ms |
|  | NOSMap | 10,000,000 | 0 (inf%) | 0 (inf%) | 5.0915601 s |
|  | HashMap | 10,000,000 | 0 (inf%) | 0 (inf%) | 6.7179596 s |
|  | NOSMap | 303,872 | 0 (inf%) | 0 (inf%) | 204.7688 ms |
|  | HashMap | 303,872 | 0 (inf%) | 0 (inf%) | 135.0235 ms |
| **Resizing w/ missing** | NOSMap | 1,000,000 | 125,000 (8.00%) | 0 (inf%) | 503.0182 ms |
|  | HashMap | 1,000,000 | 125,000 (8.00%) | 0 (inf%) | 587.1675 ms |
|  | NOSMap | 10,000,000 | 1,250,000 (8.00%) | 0 (inf%) | 5.5197976 s |
|  | HashMap | 10,000,000 | 1,250,000 (8.00%) | 0 (inf%) | 7.3583459 s |
|  | NOSMap | 303,872 | 37,984 (8.00%) | 0 (inf%) | 211.3901 ms |
|  | HashMap | 303,872 | 37,984 (8.00%) | 0 (inf%) | 143.6554 ms |

**Running command:**
```
cargo r --release
```

**Benchmark output:**
```
---------- Preallocated w/o missing ----------
Time elapsed for NOSMap is: 640.7332ms | key size 1000000 | get missing size 0 (inf%) | capacity 1001102 (99.89%)
Time elapsed for HashMap is: 399.6438ms | key size 1000000 | get missing size 0 (inf%) | capacity 1144165 (87.40%)
Time elapsed for NOSMap is: 6.9721599s | key size 10000000 | get missing size 0 (inf%) | capacity 10011012 (99.89%)
Time elapsed for HashMap is: 5.5180007s | key size 10000000 | get missing size 0 (inf%) | capacity 11441647 (87.40%)
Time elapsed for NOSMap is: 130.4601ms | key size 303872 | get missing size 0 (inf%) | capacity 304207 (99.89%)
Time elapsed for HashMap is: 114.8022ms | key size 303872 | get missing size 0 (inf%) | capacity 347680 (87.40%)
---------- Preallocated w/ missing ----------
Time elapsed for NOSMap is: 702.0693ms | key size 1000000 | get missing size 125000 (8.00%) | capacity 1001102 (99.89%)
Time elapsed for HashMap is: 434.22ms | key size 1000000 | get missing size 125000 (8.00%) | capacity 1144165 (87.40%)
Time elapsed for NOSMap is: 7.2867382s | key size 10000000 | get missing size 1250000 (8.00%) | capacity 10011012 (99.89%)
Time elapsed for HashMap is: 5.7745234s | key size 10000000 | get missing size 1250000 (8.00%) | capacity 11441647 (87.40%)
Time elapsed for NOSMap is: 2.7460905s | key size 303872 | get missing size 37984 (8.00%) | capacity 304207 (99.89%)
Time elapsed for HashMap is: 120.1809ms | key size 303872 | get missing size 37984 (8.00%) | capacity 347680 (87.40%)
---------- Resizing w/o missing ----------
Time elapsed for NOSMap is: 445.2367ms | key size 1000000 | get missing size 0 (inf%) | capacity 0 (inf%)
Time elapsed for HashMap is: 540.8554ms | key size 1000000 | get missing size 0 (inf%) | capacity 0 (inf%)
Time elapsed for NOSMap is: 5.0915601s | key size 10000000 | get missing size 0 (inf%) | capacity 0 (inf%)
Time elapsed for HashMap is: 6.7179596s | key size 10000000 | get missing size 0 (inf%) | capacity 0 (inf%)
Time elapsed for NOSMap is: 204.7688ms | key size 303872 | get missing size 0 (inf%) | capacity 0 (inf%)
Time elapsed for HashMap is: 135.0235ms | key size 303872 | get missing size 0 (inf%) | capacity 0 (inf%)
---------- Resizing w/ missing ----------
Time elapsed for NOSMap is: 503.0182ms | key size 1000000 | get missing size 125000 (8.00%) | capacity 0 (inf%)
Time elapsed for HashMap is: 587.1675ms | key size 1000000 | get missing size 125000 (8.00%) | capacity 0 (inf%)
Time elapsed for NOSMap is: 5.5197976s | key size 10000000 | get missing size 1250000 (8.00%) | capacity 0 (inf%)
Time elapsed for HashMap is: 7.3583459s | key size 10000000 | get missing size 1250000 (8.00%) | capacity 0 (inf%)
Time elapsed for NOSMap is: 211.3901ms | key size 303872 | get missing size 37984 (8.00%) | capacity 0 (inf%)
Time elapsed for HashMap is: 143.6554ms | key size 303872 | get missing size 37984 (8.00%) | capacity 0 (inf%)
```
**NOSMap setting**
```rs
	pub fn new(initial_capacity: usize) -> Self {
		let initial_prime_capacity = next_prime(initial_capacity as u32) as usize;
		let one_byte_hashes = vec![u8x32::splat(0); initial_prime_capacity];
		let key_values = vec![KeyValue::default(); initial_prime_capacity];
		let resize_hashes = vec![0; initial_prime_capacity];

		Self {
			one_byte_hashes,
			key_values,
			resize_hashes,
			load: 0,
			grow_size: 5.05,
			load_factor: 0.999,
			modulo_const: uint_div_const(initial_prime_capacity as u64) as usize,
			worst_probe: 0
		}
	}
```
**Benchmark code**
```rs
fn benchmark_2(keys: Vec<Vec<u8>>, test_capacity: usize, get_missing_size: usize) {
	{
		let start = Instant::now();

		let capacity = (test_capacity as f32 / 0.9989).ceil() as usize;
		let mut map = NOSMap::<i32>::new(capacity);
		for (i, key) in keys.clone().into_iter().enumerate() {
			map.put(key.clone(), i as i32);
		}

		for (i, key) in keys.clone().into_iter().enumerate() {
			match map.get(&key) {
				Some(index) if index == i as i32 => (),
				Some(index) => {
					// println!("MISMATCH:\nleft: {} | {:?}\nright: {} | {:?}", i, keys[i], index, keys[index as usize]);
				}
				None => {
					panic!("Key not found in map: {:?}", key);
				}
			}
		}

		for i in 0..get_missing_size {
			map.get(&Vec::<u8>::from(format!("key{}", i)));
		}

		println!("Time elapsed for NOSMap is: {:?} | key size {} | get missing size {} ({:.2}%) | capacity {} ({:.2}%)", start.elapsed(), keys.len(), get_missing_size, keys.len() as f32 / get_missing_size as f32, capacity, keys.len() as f32 / capacity as f32 * 100.0);
	}
	{
		let start = Instant::now();

		let capacity = (test_capacity as f32 / 0.874).ceil() as usize;
		let mut map = HashMap::with_capacity(capacity);
		for (i, key) in keys.clone().into_iter().enumerate() {
			map.insert(key.clone(), i as i32);
		}

		for (i, key) in keys.clone().into_iter().enumerate() {
			match map.get(&key) {
				Some(&index) if index == i as i32 => (),
				Some(&index) => {
					// println!("MISMATCH:\nleft: {} | {:?}\nright: {} | {:?}", i, keys[i], index, keys[index as usize]);
				}
				None => {
					panic!("Key not found in map: {:?}", key);
				}
			}
		}

		for i in 0..get_missing_size {
			map.get(&Vec::<u8>::from(format!("key{}", i)));
		}

		println!("Time elapsed for HashMap is: {:?} | key size {} | get missing size {} ({:.2}%) | capacity {} ({:.2}%)", start.elapsed(), keys.len(), get_missing_size, keys.len() as f32 / get_missing_size as f32, capacity, keys.len() as f32 / capacity as f32 * 100.0);
	}
}
```

# 🎇🎆 Gain
- High correctness and reliability - NOSMap's features are minimal and battle-fuzz tested with passwords
	- [Top304Thousand-probable-v2.txt](https://weakpass.com/wordlist/1859)
	- [hk_hlm_founds.txt](https://weakpass.com/wordlist/1256)
- Possibly read-only concurrent hash map with write lock
- NOSMap will only slow down by bucket collisions when the load factor is 95% above.
	- Please refrain from setting the load factor below 95% because it slows NOSMap down.
- NOSMap will drain [`insert()` hash flooding attacks](https://en.wikipedia.org/wiki/Collision_attack) until memory has exhausted
	- To collide with NOSMap's key and force a full key check, the first byte of the character and hash must match with another first byte of key and hash. Otherwise, NOSMap will rejects flooding attacks with a single hash byte check only. But, it's harder to exploit bucket collisions than to simply overload NOSMap with unique keys because the NOSMap's probing mechanism is dependent on the hash function used and the key. VashHash-b, which sums keys via AVX2, it's very unsecure but fast.
	- A 50% chance of a successful brute force attack to a compromised hash function requires 301.42 attempts, and each additional collision needs another 301.42 trials. With a blind brute force, the difficulty increases significantly from 16-bit (501.42 tries) to 64-bit (5056937540.69 tries).
	- You can tweak VashHash-b a little and NOSMap's dynamic linear probing will happily obfuscate VashHash-b, so, no one would know how to flood NOSMap without the source code.
	- NOSMap can't defend `find()` missing flooding.

**Bucket collisions mathematical proof**
```py
import math

def calculate_collision_tries(n_bits, probability=0.5):
	n = 2 ** n_bits
	ln_probability = math.log(1 - probability)
	k_approx = math.sqrt(-2 * n * ln_probability)
	return k_approx

n_bits = 16  # 2 bytes
probability = 0.5
k = calculate_collision_tries(n_bits, probability) # 301.42
print(f"Number of tries for a {probability * 100.0:.2f}% chance of collision with a {n_bits}-bit hash: {k:.2f}")

n_bits = 64  # 8 bytes
k = calculate_collision_tries(n_bits, probability) # 5056937540.69
print(f"Number of tries for a {probability * 100.0:.2f}% chance of collision with a {n_bits}-bit hash: {k:.2f}")
```

# 🚤🔥 Drawbacks - Reason
- Written in Rust instead of C - My skill issues 😭😭😭 ("I cannot fix `void *` from SIGSEGV in C.")
- Need a decompiler to read the code 🗿🙄👽
- 15% slower than the Rust hash map; could be much slower than 50% in edge cases
- Hard to debug even in Rust - The dynamic linear probing mechanism is borderline nondeterministic
- Very slow in `find()` missing - Wasn't able to determine if a key exist, I'll be rushing to improve it

# 🧻🤣🤣🤣 References
I learned new buzzwords to say. This is my name-dropping exercise. 😇

[Faster than Rust and C++: the PERFECT hash table](https://youtu.be/DMQ_HcNSOAI)  
[Array of structures (AoS) and structure of arrays (SoA)](https://en.wikipedia.org/wiki/AoS_and_SoA#Structure_of_arrays)  
[Optimizing Open Addressing - thenumb.at](https://thenumb.at/Hashtables/)  
[C++Now 2018: You Can Do Better than std::unordered_map: New Improvements to Hash Table Performance](https://youtu.be/M2fKMP47slQ)  
[CppCon 2016: Timur Doumler “Want fast C++? Know your hardware!"](https://youtu.be/BP6NxVxDQIs)  
[dendibakh/perf-book](https://github.com/dendibakh/perf-book)  
[Big O myths busted! (Time complexity is complicated)](https://youtu.be/7VHG6Y2QmtM)  
[Understanding B-Trees: The Data Structure Behind Modern Databases](https://youtu.be/K1a2Bk8NrYQ)  
[computers suck at division (a painful discovery)](https://youtu.be/ssDBqQ5f5_0)  

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

The key will be hashed by VastHash-b, then uses the first byte of the byte as dynamic hashing to decide the next index for dynamic linear probe to find an empty bucket when keys are collided, and insert into a bucket.

VastHash-b takes a `&[u64x4]` and summing it up. The distribution quality of VastHash-b was better than DJB2, but excels most algorithms with prime-sized vector.

The dynamic hashing is designed to be resistant to clustering than dynamic linear probing because NOSMap drank a fire-resistant potion 🧪. And the shorter travel of it provides higher spatial locality than double hashing. The dependency on keys and hash functions made it outdo on collision avoidance and unpredictability, so NOSMap can handle over 95% of loads without slowing down. It creates obfuscation on the hash function by mixing with the hash in the probing index along with the key.

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
		next_stride = (ord(key[0]) + (index & 0x3ff)) * 2 + 1

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
