use std::{mem, cmp::max, fmt::Debug, simd::{u8x32, prelude::SimdPartialEq}};
use crate::{vasthash_b::*, is_prime::*};


const EMPTY: u8 = 0;
const OCCUPIED: u8 = 0b1;
const TOMESTONE: u8 = 0b10;

pub fn find_leftmost_avx2(input: u8x32, cmp: u8x32) -> u32 {
	input.simd_eq(cmp).to_bitmask().trailing_zeros() // vpcmpeqd, vmovmskps, bsf
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct KeyValue<V> {
	pub key: Vec<u8>,
	pub value: V,
}

/// # NOSMap
/// NOSMap has a much slower resizing time than probing time.
/// - The size should be pre-allocated.
/// - NOSMap's speed is determined by resize count.
/// - NOSMap will only start slowing down by bucket collisions at 95% above load factor, so, lowering the load factor will slow down NOSMap due to resizing time
/// ## Performance Explanation
/// - `grow_size` effects NOSMap's performance the most.
/// - `initial_capacity` effects NOSMap's performance because of resizing.
/// - `load_factor` effects NOSMap's performance stablity.
/// ## Recommend setting
/// ### 1m Elements
/// - initial_capacity: 1
/// - grow_size 5.05
/// - load_factor 0.97
#[derive(Debug)]
pub struct NOSMap<V> {
	pub one_byte_hashes: Vec<u8x32>,
	pub key_values: Vec<KeyValue<V>>,
	pub resize_hashes: Vec<u64>,

	pub load: usize,
	pub grow_size: f32,
	pub load_factor: f32,
	modulo_const: usize
}

impl<V: Clone + Default + PartialEq + Debug> NOSMap<V> {
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
			load_factor: 0.97,
			modulo_const: uint_div_const(initial_prime_capacity as u64) as usize
		}
	}

	pub fn _find_empty_bucket_hash(&self, key: &Vec<u8>, hash: u64) -> usize {
		let mut index = fast_mod(hash, self.modulo_const as u64, self.key_values.len() as u64) as usize;
		let mut next_stride = key[0] as usize + (hash & 0x3ff) as usize;

		// The AMD64 is running out of regesiter to use, so it will cause NOSMap to run much slower.
		// Please probe the hash in batch with an array, or set up an artificial boundary.
		let mut i = 0;
		loop {
			let simd_index = index / 32;
			let empty = find_leftmost_avx2(self.one_byte_hashes[simd_index], u8x32::splat(EMPTY)) as usize;

			if empty != u32::MAX as usize {
				if simd_index * 32 + empty < self.key_values.len() {
					return simd_index * 32 + empty;
				}
			}

			index += next_stride;
			while index >= self.key_values.len() {
				index -= self.key_values.len();
			}
			if i >= self.key_values.len() {
				// println!("_find_empty_bucket_hash | Index might have an infinite loop for key {:?} | index {}", key, index);
				// println!("_find_empty_bucket_hash | self.key_values {:?}", self.key_values);
				next_stride = 1;
			}
			i += 1;
		}
	}

	pub fn _find_hash_match_hash(&self, key: &Vec<u8>, hash: u64) -> Option<usize> {
		let mut index = fast_mod(hash, self.modulo_const as u64, self.key_values.len() as u64) as usize;
		let compare_hash = u8x32::splat(hash as u8 & !(OCCUPIED | TOMESTONE) | OCCUPIED);
		let mut next_stride = key[0] as usize + (hash & 0x3ff) as usize;

		let mut i = 0;
		loop {
			let simd_index = index / 32;
			let mut one_byte_simd = self.one_byte_hashes[simd_index];

			// println!("_find_hash_match_hash | one_byte_simd {:?}", one_byte_simd);
			// println!("_find_hash_match_hash | self.key_values {:?}", self.key_values);

			let mut hash_match = find_leftmost_avx2(one_byte_simd, compare_hash) as usize;
			// println!("_find_hash_match_hash | hash_match {}", hash_match);

			let empty = find_leftmost_avx2(one_byte_simd, u8x32::splat(EMPTY)) as usize;
			// println!("_find_hash_match_hash | empty {}", empty);
			if empty != u32::MAX as usize && empty < hash_match {
				if simd_index * 32 + empty < self.key_values.len() {
					break;
				}
			}

			while hash_match != u32::MAX as usize && simd_index * 32 + hash_match < self.key_values.len()  {
				let key_index = simd_index * 32 + hash_match;
				// println!("_find_hash_match_hash | key {:?}", key);
				// println!("_find_hash_match_hash | self.key_values[key_index].key {:?}", self.key_values[key_index].key);
				if *key == self.key_values[key_index].key {
					// println!("_find_hash_match_hash | Some");
					return Some(key_index)
				}
				one_byte_simd[hash_match] = 0;
				hash_match = find_leftmost_avx2(one_byte_simd, compare_hash) as usize;
			}

			index += next_stride;
			while index >= self.key_values.len() {
				index -= self.key_values.len();
			}
			if i >= self.key_values.len() {
				// println!("_find_hash_match_hash | Index might have an infinite loop for key {:?} | index {}", key, index);
				next_stride = 1;
			}
			i += 1;
		}
	return None;
	}

	pub fn _find_empty_bucket_string(&self, key: &Vec<u8>) -> (usize, u64) {
		let hash = hash_u8(key);
		let index = self._find_empty_bucket_hash(key, hash);
		(index, hash)
	}

	pub fn _find_hash_match_string(&self, key: &Vec<u8>) -> Option<usize> {
		let hash = hash_u8(key);
		self._find_hash_match_hash(key, hash)
	}

	pub fn _put_only(&mut self, key: Vec<u8>, value: V, hash: u64, index: usize) {
		self.one_byte_hashes[index / 32][index % 32] = hash as u8 & !(OCCUPIED | TOMESTONE) | OCCUPIED;
		self.resize_hashes[index] = hash;
		self.key_values[index] = KeyValue{key, value};
		self.load += 1;
	}

	pub fn put(&mut self, key: Vec<u8>, value: V) {
		let (index, hash) = self._find_empty_bucket_string(&key);
		Self::_put_only(self, key, value, hash, index);

		if self.load > (self.key_values.len() as f32 * self.load_factor) as usize {
			self._auto_resize();
		}
	}

	pub fn _auto_resize(&mut self) {
		let new_capacity = max(self.key_values.len() + 1, (self.key_values.len() as f32 * self.grow_size).ceil() as usize);
		self._resize(new_capacity);
	}

	pub fn get(&self, key: &Vec<u8>) -> Option<V> {
		let index = self._find_hash_match_string(&key);
		index.and_then(|x| Some(self.key_values[x].value.clone()))
	}

	pub fn remove(&mut self, key: &Vec<u8>) {
		let index = self._find_hash_match_string(&key);
		match index {
			Some(i) => {
				self.one_byte_hashes[i / 32][i % 32] = TOMESTONE;
				self.key_values[i] = KeyValue::default();
				self.resize_hashes[i] = 0;
			}
			_ => ()
		}
	}

	pub fn _resize(&mut self, new_capacity: usize) {
		let new_prime_capacity = next_prime(new_capacity as u32) as usize;
		self.modulo_const = uint_div_const(new_prime_capacity as u64) as usize;
		self.load = 0;
		let mut old_one_byte_hashes = mem::replace(&mut self.one_byte_hashes, vec![u8x32::splat(0); new_prime_capacity]);
		let old_key_values = mem::replace(&mut self.key_values, vec![KeyValue::default(); new_prime_capacity]);
		let old_resize_hashes = mem::replace(&mut self.resize_hashes, vec![0; new_prime_capacity]);

		for old_simd_index in 0..old_one_byte_hashes.len() {
			loop {
				let next_occupied = find_leftmost_avx2(old_one_byte_hashes[old_simd_index] & u8x32::splat(OCCUPIED), u8x32::splat(OCCUPIED)) as usize;

				if next_occupied == u32::MAX as usize {
					break;
				}

				old_one_byte_hashes[old_simd_index][next_occupied] = EMPTY;

				let old_index = old_simd_index * 32 + next_occupied;
				// println!("_resize | old_simd_index {}", old_simd_index);
				// println!("_resize | next_occupied {}", next_occupied);
				// println!("_resize | old_one_byte_hashes[old_simd_index] {:?}", old_one_byte_hashes[old_simd_index]);
				let key = old_key_values[old_index].key.clone();
				let value = old_key_values[old_index].value.clone();
				let resize_hash = old_resize_hashes[old_index];

				let index = self._find_empty_bucket_hash(&key, resize_hash);
				Self::_put_only(self, key, value, resize_hash, index);
			}
		}
	}
}


#[cfg(test)]
mod tests {
	use super::*;
	use std::time::Instant;

	#[test]
	#[should_panic]
	fn test_empty_map() {
		let map = NOSMap::<i32>::new(0);
		assert_eq!(map.key_values.len(), 0);
		assert_eq!(map.one_byte_hashes.len(), 0);
		assert_eq!(map.resize_hashes.len(), 0);
		assert_eq!(map.load, 0);
	}

	#[test]
	fn test_single_element() {
		let mut map = NOSMap::<i32>::new(1);
		map.put(Vec::<u8>::from("key"), 42);

		assert_eq!(map.get(&Vec::<u8>::from("key")), Some(42));
		assert_eq!(map.load, 1);
	}

	#[test]
	fn test_resize_on_insert() {
		let mut map = NOSMap::<i32>::new(2);

		for i in 0..35 {
			map.put(Vec::<u8>::from(format!("key{}", i)), i); // This should trigger a resize
		}

		assert!(map.key_values.len() > 35);
		assert!(map.one_byte_hashes.len() > 35);
		assert!(map.resize_hashes.len() > 35);
	}

	#[test]
	fn test_find_buckets_string_collision() {
		let mut map = NOSMap::<i32>::new(2);
		map.put(Vec::<u8>::from("key1"), 1);
		map.put(Vec::<u8>::from("key2"), 2);

		let value1 = map.get(&Vec::<u8>::from("key1"));
		let value2 = map.get(&Vec::<u8>::from("key2"));

		assert_ne!(Some(value1), Some(value2));
	}

	#[test]
	fn test_put_same_key() {
		let mut map = NOSMap::<i32>::new(2);
		map.put(Vec::<u8>::from("key"), 1);
		map.put(Vec::<u8>::from("key"), 2); // Overwrite the value

		assert_eq!(map.get(&Vec::<u8>::from("key")), Some(2));
	}

	/// `cargo test test_large_capacity --release`
	#[test]
	fn test_large_capacity() {
		let mut keys = Vec::with_capacity(1_000);
		for i in 100_000..101_000 {
			keys.push(Vec::<u8>::from(format!("key{}", i)));
		}
		{
			let start = Instant::now();

			let mut map = NOSMap::<i32>::new(1);
			for (i, key) in keys.clone().into_iter().enumerate() {
				map.put(key.clone(), i as i32);
				assert_eq!(map.get(&key), Some(i as i32));
			}

			println!("Time elapsed for NOSMap is: {:?}", start.elapsed());
		}
	}

	#[test]
	fn test_get() {
		let mut map = NOSMap::new(1);
		let key = vec![1, 2, 3];
		let value = "test_value".to_string();
		map.put(key.clone(), value.clone());

		let result = map.get(&key);
		assert_eq!(result, Some(value));

		let non_existent_key = vec![4, 5, 6];
		let result = map.get(&non_existent_key);
		assert_eq!(result, None);
	}

	#[test]
	fn test_remove() {
		let mut map = NOSMap::new(1);
		let key = vec![1, 2, 3];
		let value = "test_value".to_string();
		map.put(key.clone(), value.clone());

		map.remove(&key);
		let result = map.get(&key);
		assert_eq!(result, None);

		let non_existent_key = vec![4, 5, 6];
		map.remove(&non_existent_key);
		let result = map.get(&non_existent_key);
		assert_eq!(result, None);
	}
}
