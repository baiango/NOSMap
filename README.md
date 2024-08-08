# 🤗✨ NOSmap - Dinitrogen oxide Hashmap
NOSMap is a virtual homework experimental AVX2 accelerated hashmap ​project that aims to have a 99.9% load factor while being fast for low-latency number crunching and memory-intensive data processing algorithms at disregard of quadratic implementation difficulty and portability. However, multiple mini unit tests are made to ensure correctness of NOSMap.

NOSMap will speed up training tokenizer by minimizing memory reads and writes, reducing memory usage and computation to the minimum possible.

NOSMap was designed for to end the users' search for the next fastest Hashmap on AMD64 for few years.

NOSMap will also ace most of the section on benchmarks from people; only if you can compile it, and the chance is 100.0% on AVX2 system because of the ease of the Rust installation.

# 🫠🌪️🏳️ Performance
I am completely devastated by Rust hash map performance. My NOSMap's design could not beat the Rust hash map when preallocated both hash maps. I will declare defeat. I am done with this hash map. 😭🙇‍♀️ A 99.9% load for a hash map? It's just an overengineered array. I'ma start with NOSStack.

| Scenario | Map Type | Key Size | Missing Size (Percentage) | Initial Capacity | Time Elapsed (ms/s) |
|---|---|---|---|---|---|
| **Preallocated w/o missing** | NOSMap | 1000000 | 0 (0.00%) | 1001102 (99.89%) | 643.9751ms
| | HashMap | 1000000 | 0 (0.00%) | 1144165 (87.40%) | 367.0978ms
| | NOSMap | 10000000 | 0 (0.00%) | 10011012 (99.89%) | 6.9044502s
| | HashMap | 10000000 | 0 (0.00%) | 11441647 (87.40%) | 5.1444732s
| | NOSMap | 80000000 | 0 (0.00%) | 80088096 (99.89%) | 205.6192116s
| | HashMap | 80000000 | 0 (0.00%) | 91533176 (87.40%) | 53.5207945s
| | NOSMap | [303872](https://weakpass.com/wordlist/1859) | 0 (0.00%) | 304207 (99.89%) | 173.1047ms
| | HashMap | [303872](https://weakpass.com/wordlist/1859) | 0 (0.00%) | 347680 (87.40%) | 104.6524ms
| | NOSMap | [38647798](https://weakpass.com/wordlist/1256) | 0 (0.00%) | 38690360 (99.89%) | 120.2457072s
| | HashMap | [38647798](https://weakpass.com/wordlist/1256) | 0 (0.00%) | 44219452 (87.40%) | 31.3383805s
| **Preallocated w/ missing** | NOSMap | 1000000 | 125000 (12.50%) | 1001102 (99.89%) | 681.8579ms
| | HashMap | 1000000 | 125000 (12.50%) | 1144165 (87.40%) | 421.4585ms
| | NOSMap | 10000000 | 1250000 (12.50%) | 10011012 (99.89%) | 9.4380203s
| | HashMap | 10000000 | 1250000 (12.50%) | 11441647 (87.40%) | 6.4654762s
| | NOSMap | 80000000 | 10000000 (12.50%) | 80088096 (99.89%) | 218.7643619s
| | HashMap | 80000000 | 10000000 (12.50%) | 91533176 (87.40%) | 66.380596s
| | NOSMap | [303872](https://weakpass.com/wordlist/1859) | 37984 (12.50%) | 304207 (99.89%) | 2.83666s
| | HashMap | [303872](https://weakpass.com/wordlist/1859) | 37984 (12.50%) | 347680 (87.40%) | 112.8536ms
| | NOSMap | [38647798](https://weakpass.com/wordlist/1256) | 386 (0.00%) | 38690360 (99.89%) | 134.2869561s
| | HashMap | [38647798](https://weakpass.com/wordlist/1256) | 386 (0.00%) | 44219452 (87.40%) | 40.2543792s
| **Resizing w/o missing** | NOSMap | 1000000 | 0 (0.00%) | 0 (inf%) | 436.3151ms
| | HashMap | 1000000 | 0 (0.00%) | 0 (inf%) | 557.3059ms
| | NOSMap | 10000000 | 0 (0.00%) | 0 (inf%) | 9.1041956s
| | HashMap | 10000000 | 0 (0.00%) | 0 (inf%) | 7.8982911s
| | NOSMap | 80000000 | 0 (0.00%) | 0 (inf%) | 63.020101s
| | HashMap | 80000000 | 0 (0.00%) | 0 (inf%) | 80.0118641s
| | NOSMap | [303872](https://weakpass.com/wordlist/1859) | 0 (0.00%) | 0 (inf%) | 200.1249ms
| | HashMap | [303872](https://weakpass.com/wordlist/1859) | 0 (0.00%) | 0 (inf%) | 133.9257ms
| | NOSMap | [38647798](https://weakpass.com/wordlist/1256) | 0 (0.00%) | 0 (inf%) | 75.9037819s
| | HashMap | [38647798](https://weakpass.com/wordlist/1256) | 0 (0.00%) | 0 (inf%) | 46.4104112s
| **Resizing w/ missing** | NOSMap | 1000000 | 10000 (1.00%) | 0 (inf%) | 472.4232ms
| | HashMap | 1000000 | 10000 (1.00%) | 0 (inf%) | 538.1744ms
| | NOSMap | 10000000 | 100000 (1.00%) | 0 (inf%) | 9.2910668s
| | HashMap | 10000000 | 100000 (1.00%) | 0 (inf%) | 8.0348904s
| | NOSMap | 80000000 | 800000 (1.00%) | 0 (inf%) | 60.6304244s
| | HashMap | 80000000 | 800000 (1.00%) | 0 (inf%) | 82.1701226s
| | NOSMap | [303872](https://weakpass.com/wordlist/1859) | 3038 (1.00%) | 0 (inf%) | 214.6126ms
| | HashMap | [303872](https://weakpass.com/wordlist/1859) | 3038 (1.00%) | 0 (inf%) | 135.4813ms
| | NOSMap | [38647798](https://weakpass.com/wordlist/1256) | 386 (0.00%) | 0 (inf%) | 78.2482301s
| | HashMap | [38647798](https://weakpass.com/wordlist/1256) | 386 (0.00%) | 0 (inf%) | 48.0270358s

**Running command:**
```
cargo r --release
```

**Raw benchmark output:**
```
---------- Loading file ----------
| Scenario | Map Type | Key Size | Missing Size (Percentage) | Initial Capacity | Time Elapsed (ms/s) |
|---|---|---|---|---|---|
| **Preallocated w/o missing** | NOSMap | 1000000 | 0 (0.00%) | 1001102 (99.89%) | 643.9751ms
| HashMap | 1000000 | 0 (0.00%) | 1144165 (87.40%) | 367.0978ms
| NOSMap | 10000000 | 0 (0.00%) | 10011012 (99.89%) | 6.9044502s
| HashMap | 10000000 | 0 (0.00%) | 11441647 (87.40%) | 5.1444732s
| NOSMap | 80000000 | 0 (0.00%) | 80088096 (99.89%) | 205.6192116s
| HashMap | 80000000 | 0 (0.00%) | 91533176 (87.40%) | 53.5207945s
| NOSMap | 303872 | 0 (0.00%) | 304207 (99.89%) | 173.1047ms
| HashMap | 303872 | 0 (0.00%) | 347680 (87.40%) | 104.6524ms
| NOSMap | 38647798 | 0 (0.00%) | 38690360 (99.89%) | 120.2457072s
| HashMap | 38647798 | 0 (0.00%) | 44219452 (87.40%) | 31.3383805s
| **Preallocated w/ missing** | NOSMap | 1000000 | 125000 (12.50%) | 1001102 (99.89%) | 681.8579ms
| HashMap | 1000000 | 125000 (12.50%) | 1144165 (87.40%) | 421.4585ms
| NOSMap | 10000000 | 1250000 (12.50%) | 10011012 (99.89%) | 9.4380203s
| HashMap | 10000000 | 1250000 (12.50%) | 11441647 (87.40%) | 6.4654762s
| NOSMap | 80000000 | 10000000 (12.50%) | 80088096 (99.89%) | 218.7643619s
| HashMap | 80000000 | 10000000 (12.50%) | 91533176 (87.40%) | 66.380596s
| NOSMap | 303872 | 37984 (12.50%) | 304207 (99.89%) | 2.83666s
| HashMap | 303872 | 37984 (12.50%) | 347680 (87.40%) | 112.8536ms
| NOSMap | 38647798 | 386 (0.00%) | 38690360 (99.89%) | 134.2869561s
| HashMap | 38647798 | 386 (0.00%) | 44219452 (87.40%) | 40.2543792s
| **Resizing w/o missing** | NOSMap | 1000000 | 0 (0.00%) | 0 (inf%) | 436.3151ms
| HashMap | 1000000 | 0 (0.00%) | 0 (inf%) | 557.3059ms
| NOSMap | 10000000 | 0 (0.00%) | 0 (inf%) | 9.1041956s
| HashMap | 10000000 | 0 (0.00%) | 0 (inf%) | 7.8982911s
| NOSMap | 80000000 | 0 (0.00%) | 0 (inf%) | 63.020101s
| HashMap | 80000000 | 0 (0.00%) | 0 (inf%) | 80.0118641s
| NOSMap | 303872 | 0 (0.00%) | 0 (inf%) | 200.1249ms
| HashMap | 303872 | 0 (0.00%) | 0 (inf%) | 133.9257ms
| NOSMap | 38647798 | 0 (0.00%) | 0 (inf%) | 75.9037819s
| HashMap | 38647798 | 0 (0.00%) | 0 (inf%) | 46.4104112s
| **Resizing w/ missing** | NOSMap | 1000000 | 10000 (1.00%) | 0 (inf%) | 472.4232ms
| HashMap | 1000000 | 10000 (1.00%) | 0 (inf%) | 538.1744ms
| NOSMap | 10000000 | 100000 (1.00%) | 0 (inf%) | 9.2910668s
| HashMap | 10000000 | 100000 (1.00%) | 0 (inf%) | 8.0348904s
| NOSMap | 80000000 | 800000 (1.00%) | 0 (inf%) | 60.6304244s
| HashMap | 80000000 | 800000 (1.00%) | 0 (inf%) | 82.1701226s
| NOSMap | 303872 | 3038 (1.00%) | 0 (inf%) | 214.6126ms
| HashMap | 303872 | 3038 (1.00%) | 0 (inf%) | 135.4813ms
| NOSMap | 38647798 | 386 (0.00%) | 0 (inf%) | 78.2482301s
| HashMap | 38647798 | 386 (0.00%) | 0 (inf%) | 48.0270358s
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
fn benchmark_2(keys: Vec<Vec<u8>>, test_capacity: usize, missing_size: usize) {
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

		for i in 0..missing_size {
			map.get(&Vec::<u8>::from(format!("key{}", i)));
		}

		println!("| NOSMap | {} | {} ({:.2}%) | {} ({:.2}%) | {:?}", keys.len(), missing_size, missing_size as f32 / keys.len() as f32 * 100.0, capacity, keys.len() as f32 / capacity as f32 * 100.0, start.elapsed());
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

		for i in 0..missing_size {
			map.get(&Vec::<u8>::from(format!("key{}", i)));
		}

		println!("| HashMap | {} | {} ({:.2}%) | {} ({:.2}%) | {:?}", keys.len(), missing_size, missing_size as f32 / keys.len() as f32 * 100.0, capacity, keys.len() as f32 / capacity as f32 * 100.0, start.elapsed());
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
- A toy hash map with linear probing using a sum toy hash 🗿🙄👽
- 15% slower than the Rust hash map; could be 100% slower in edge cases
- Hard to debug even in Rust - The dynamic linear probing mechanism is borderline nondeterministic
- Very slow in `find()` missing - NOSMap wasn't able to determine if a key exist, I'll be rushing to improve it
- Low probe scalability - It uses linear probe, but should be enough for handling 200k elements

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
[ktprime/emhash](https://github.com/ktprime/emhash)

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
