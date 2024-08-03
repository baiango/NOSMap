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
		// NOSMap bug hunting
		// if i >= 1_000_000 {
		// 	break;
		// }
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
