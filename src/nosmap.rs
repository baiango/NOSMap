use std::{mem, cmp::max, fmt::Debug};
use crate::vasthash_b::*;


const EMPTY: u8 = 0;
const OCCUPIED: u8 = 0b1;
const TOMESTONE: u8 = 0b10;

fn uint_div_const(div: usize) -> usize {
	(1 << 63) / div
}

pub fn fast_mod(num: usize, div_const: usize, modulo: usize) -> usize {
	let mut result = (num as u128 - (num as u128 * div_const as u128 >> 63) * modulo as u128) as usize;
	if result >= modulo {
		result -= modulo;
	}
	result
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct KeyValue<V> {
	pub key: Vec<u8>,
	pub value: V,
}

#[derive(Debug)]
pub struct NOSMap<V> {
	pub one_byte_hashes: Vec<u8>,
	pub key_values: Vec<KeyValue<V>>,
	pub resize_hashes: Vec<u64>,

	pub load: usize,
	pub grow_size: f32,
	pub load_factor: f32,
}

impl<V> Default for NOSMap<V> {
	fn default() -> Self {
		Self {
			one_byte_hashes: vec![],
			key_values: vec![],
			resize_hashes: vec![],
			load: 0,
			grow_size: 1.618,
			load_factor: 0.9
		}
	}
}

impl<V: Clone + Default + PartialEq + Debug> NOSMap<V> {
	pub fn new(initial_capacity: usize) -> Self {
		if initial_capacity == 0 {
			panic!("initial_capacity is 0!");
		}
		let one_byte_hashes = vec![0; initial_capacity];
		let key_values = vec![KeyValue::default(); initial_capacity];
		let resize_hashes = vec![0; initial_capacity];

		Self {
			one_byte_hashes,
			key_values,
			resize_hashes,
			load: 0,
			grow_size: 1.618 * 2.0,
			load_factor: 0.9
		}
	}

	pub fn _find_buckets_hash(&self, key: &Vec<u8>, hash: u64) -> (usize, bool) {
		let div_const = uint_div_const(self.key_values.len());
		let mut index = fast_mod(hash as usize, div_const, self.key_values.len());
		let mut stride_traveled = 0;

		while self.one_byte_hashes[index] & (OCCUPIED | TOMESTONE) != EMPTY {
			if hash as u8 & !(OCCUPIED | TOMESTONE) | OCCUPIED == self.one_byte_hashes[index]
			&& *key == self.key_values[index].key {
				return (index, true);
			}

			let next_stride = key[0] as usize
				- (stride_traveled / self.key_values.len());
			stride_traveled += next_stride;
			index = fast_mod(index + next_stride, div_const, self.key_values.len());
		}
		(index, false)
	}

	pub fn _find_buckets_string(&self, key: &Vec<u8>) -> (usize, u64, bool) {
		let hash = hash_u8(key);
		let (index, found) = self._find_buckets_hash(key, hash);
		(index, hash, found)
	}

	pub fn put(&mut self, key: Vec<u8>, value: V) {
		let (index, hash, _) = self._find_buckets_string(&key);
		self.one_byte_hashes[index] = hash as u8 & !(OCCUPIED | TOMESTONE) | OCCUPIED;
		self.resize_hashes[index] = hash;
		self.key_values[index] = KeyValue{key, value};
		self.load += 1;

		if self.load > (self.key_values.len() as f32 * self.load_factor) as usize {
			let new_capacity = max(self.key_values.len() + 1, (self.key_values.len() as f32 * self.grow_size).ceil() as usize);
			self._resize(new_capacity);
		}
	}

	#[inline]
	pub fn _put_only(&mut self, key: Vec<u8>, value: V, hash: u64, index: usize) {
		self.one_byte_hashes[index] = hash as u8 & !(OCCUPIED | TOMESTONE) | OCCUPIED;
		self.resize_hashes[index] = hash;
		self.key_values[index] = KeyValue{key, value};
		self.load += 1;
	}

	pub fn _resize(&mut self, new_capacity: usize) {
		self.load = 0;
		let old_one_byte_hashes = mem::replace(&mut self.one_byte_hashes, vec![0; new_capacity]);
		let old_key_values = mem::replace(&mut self.key_values, vec![KeyValue::default(); new_capacity]);
		let old_resize_hashes = mem::replace(&mut self.resize_hashes, vec![0; new_capacity]);

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
		for i in 1..1_000_000 {
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
}
