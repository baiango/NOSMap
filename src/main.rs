#![feature(portable_simd)]
use std::{time::Instant, collections::HashMap};
mod nosmap;
mod vasthash_b;
mod is_prime;
use nosmap::NOSMap;


fn main() {
	let mut keys = Vec::with_capacity(1_000_000);
	for i in 0..1_000_000 {
		keys.push(Vec::<u8>::from(format!("key{}", i)));
	}
	{
		let start = Instant::now();

		let mut map = NOSMap::<i32>::new((1_000_000.0 / 0.969) as usize);
		for (i, key) in keys.clone().into_iter().enumerate() {
			map.put(key.clone(), i as i32);
			let (index, _, _) = map._find_buckets_string(&key);
			assert_eq!(map.key_values[index].value, i as i32);
		}

		println!("Time elapsed for NOSMap is: {:?}", start.elapsed());
	}
	{
		let start = Instant::now();

		let mut map = HashMap::with_capacity((1_000_000.0 / 0.874) as usize);
		for (i, key) in keys.clone().into_iter().enumerate() {
			map.insert(key.clone(), i as i32);
			assert_eq!(map.get(&key), Some(&(i as i32)));
		}

		let duration = start.elapsed();

		println!("Time elapsed for HashMap is: {:?}", duration);
	}
}
