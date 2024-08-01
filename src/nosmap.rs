use std::{mem, cmp::max, fmt::Debug};
use crate::{vasthash_b::*, is_prime::*};


const EMPTY: u8 = 0;
const OCCUPIED: u8 = 0b1;
const TOMESTONE: u8 = 0b10;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct KeyValue<V> {
	pub key: Vec<u8>,
	pub value: V,
}

/// # NOSMap
/// NOSMap has a much slower resizing time than probing time.
/// - The size should be pre-allocated.
/// ## Performance Explanation
/// - `grow_size` effects NOSMap's performance the most.
/// - `initial_capacity` effects NOSMap's performance because of resizing.
/// - `load_factor` effects NOSMap's performance stablity.
/// ## Recommend setting
/// ### 300k Elements
/// - initial_capacity: 1
/// - grow_size: 1.618 * 4.97
/// - load_factor: 0.999
/// ### 1m Elements
/// - initial_capacity: 1
/// - grow_size 5.45,
/// - load_factor 0.999
#[derive(Debug)]
pub struct NOSMap<V> {
	pub one_byte_hashes: Vec<u8>,
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
		let one_byte_hashes = vec![0; initial_prime_capacity];
		let key_values = vec![KeyValue::default(); initial_prime_capacity];
		let resize_hashes = vec![0; initial_prime_capacity];

		Self {
			one_byte_hashes,
			key_values,
			resize_hashes,
			load: 0,
			grow_size: 5.45,
			load_factor: 0.95,
			modulo_const: uint_div_const(initial_prime_capacity as u64) as usize
		}
	}

	pub fn _find_buckets_hash(&self, key: &Vec<u8>, hash: u64) -> (usize, bool) {
		let mut index = fast_mod(hash, self.modulo_const as u64, self.key_values.len() as u64) as usize;
		let compare_hash = hash as u8;
		let next_stride = compare_hash as usize;

		while self.one_byte_hashes[index] & (OCCUPIED | TOMESTONE) != EMPTY {
			if compare_hash & !(OCCUPIED | TOMESTONE) | OCCUPIED == self.one_byte_hashes[index]
			&& *key == self.key_values[index].key {
				return (index, true)
			}

			index += next_stride;
			while index >= self.key_values.len() {
				index -= self.key_values.len();
			}
		}
		(index, false)
	}

	pub fn _find_buckets_string(&self, key: &Vec<u8>) -> (usize, u64, bool) {
		let hash = hash_u8(key);
		let (index, found) = self._find_buckets_hash(key, hash);
		(index, hash, found)
	}

	pub fn _put_only(&mut self, key: Vec<u8>, value: V, hash: u64, index: usize) {
		self.one_byte_hashes[index] = hash as u8 & !(OCCUPIED | TOMESTONE) | OCCUPIED;
		self.resize_hashes[index] = hash;
		self.key_values[index] = KeyValue{key, value};
		self.load += 1;
	}

	pub fn put(&mut self, key: Vec<u8>, value: V) {
		let (index, hash, _) = self._find_buckets_string(&key);
		Self::_put_only(self, key, value, hash, index);

		if self.load > (self.key_values.len() as f32 * self.load_factor) as usize {
			let new_capacity = max(self.key_values.len() + 1, (self.key_values.len() as f32 * self.grow_size).ceil() as usize);
			self._resize(new_capacity);
		}
	}

	pub fn get(&self, key: &Vec<u8>) -> Option<V> {
		let (index, _, found) = self._find_buckets_string(&key);
		found.then(|| self.key_values[index].value.clone())
	}

	pub fn remove(&mut self, key: &Vec<u8>) {
		let (index, _, found) = self._find_buckets_string(&key);
		if found {
			self.one_byte_hashes[index] = TOMESTONE;
			self.key_values[index] = KeyValue::default();
			self.resize_hashes[index] = 0;
		}
	}

	pub fn _resize(&mut self, new_capacity: usize) {
		let new_prime_capacity = next_prime(new_capacity as u32) as usize;
		self.modulo_const = uint_div_const(new_prime_capacity as u64) as usize;
		self.load = 0;
		let old_one_byte_hashes = mem::replace(&mut self.one_byte_hashes, vec![0; new_prime_capacity]);
		let old_key_values = mem::replace(&mut self.key_values, vec![KeyValue::default(); new_prime_capacity]);
		let old_resize_hashes = mem::replace(&mut self.resize_hashes, vec![0; new_prime_capacity]);

		for old_index in 0..old_key_values.len() {
			if old_one_byte_hashes[old_index] & OCCUPIED == OCCUPIED {
				let key = old_key_values[old_index].key.clone();
				let value = old_key_values[old_index].value.clone();
				let resize_hash = old_resize_hashes[old_index];

				let (index, _) = self._find_buckets_hash(&key, resize_hash);
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

		let (index, _, _) = map._find_buckets_string(&Vec::<u8>::from("key"));
		assert_eq!(map.key_values[index].key, Vec::<u8>::from("key"));
		assert_eq!(map.key_values[index].value, 42);
		assert_eq!(map.load, 1);
	}

	#[test]
	fn test_resize_on_insert() {
		let mut map = NOSMap::<i32>::new(2);
		map.put(Vec::<u8>::from("key1"), 1);
		map.put(Vec::<u8>::from("key2"), 2);
		map.put(Vec::<u8>::from("key3"), 3); // This should trigger a resize

		assert!(map.key_values.len() > 2);
		assert!(map.one_byte_hashes.len() > 2);
		assert!(map.resize_hashes.len() > 2);
	}

	#[test]
	fn test_find_buckets_string_collision() {
		let mut map = NOSMap::<i32>::new(2);
		map.put(Vec::<u8>::from("key1"), 1);
		map.put(Vec::<u8>::from("key2"), 2);

		let (index1, _, _) = map._find_buckets_string(&Vec::<u8>::from("key1"));
		let (index2, _, _) = map._find_buckets_string(&Vec::<u8>::from("key2"));

		assert_ne!(index1, index2);
	}

	#[test]
	fn test_put_same_key() {
		let mut map = NOSMap::<i32>::new(2);
		map.put(Vec::<u8>::from("key"), 1);
		map.put(Vec::<u8>::from("key"), 2); // Overwrite the value

		let (index, _, _) = map._find_buckets_string(&Vec::<u8>::from("key"));
		assert_eq!(map.key_values[index].value, 2);
	}

	/// `cargo test test_large_capacity --release`
	#[test]
	fn test_large_capacity() {
		let mut keys = Vec::with_capacity(1_000_000);
		for i in 1..1_000_000_0 {
			keys.push(Vec::<u8>::from(format!("key{}", i)));
		}
		{
			let start = Instant::now();

			let mut map = NOSMap::<i32>::new(1);
			for (i, key) in keys.clone().into_iter().enumerate() {
				map.put(key.clone(), i as i32);
				let (index, _, _) = map._find_buckets_string(&key);
				assert_eq!(map.key_values[index].value, i as i32);
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
