#![feature(portable_simd)]
use std::{time::Instant, collections::HashMap};
mod nosmap;
mod vasthash_b;
use nosmap::NOSMap;


fn main() {
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
	{
		let start = Instant::now();

		let mut map = HashMap::new();
		for (i, key) in keys.clone().into_iter().enumerate() {
			map.insert(key.clone(), i as i32);
			assert_eq!(map.get(&key), Some(&(i as i32)));
		}

		let duration = start.elapsed();

		println!("Time elapsed for HashMap is: {:?}", duration);
	}
}
