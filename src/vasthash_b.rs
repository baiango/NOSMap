use std::simd::{u32x8, ToBytes};


pub fn sum_u32x8_scalar(simds: u32x8) -> u32 {
	simds[0]
		.wrapping_add(simds[1])
		.wrapping_add(simds[2])
		.wrapping_add(simds[3])
		.wrapping_add(simds[4])
		.wrapping_add(simds[5])
		.wrapping_add(simds[6])
		.wrapping_add(simds[7])
}

pub fn hash(input_data: &[u32x8]) -> u32 {
	let mut hash = u32x8::splat(0);

	for i in 0..input_data.len() {
		hash += input_data[i];
	}

	sum_u32x8_scalar(hash)
}

pub fn hash_u8(input_data: &[u8]) -> u32 {
	let mut hash = u32x8::splat(0);

	let len = input_data.len();
	let full_chunks = len / 32;

	for i in 0..full_chunks {
		let start = i << 5;
		let end = start + 32;
		let chunk = &input_data[start..end];
		let data = u32x8::from_le_bytes(chunk.try_into().unwrap());

		hash += data;
	}

	let remaining_start = full_chunks * 32;
	if remaining_start < len {
		let mut data_arr = [0u8; 32];
		let remaining_len = len - remaining_start;
		data_arr[..remaining_len].copy_from_slice(&input_data[remaining_start..]);
		let data = u32x8::from_le_bytes(data_arr.into());

		hash += data;
	}

	sum_u32x8_scalar(hash)
}


#[cfg(test)]
mod tests {
	use crate::vasthash_b::*;

	#[test]
	fn test_hash() {
		let result = hash(&vec![u32x8::splat(123), u32x8::splat(123)]);
		assert_eq!(result, 984);
	}

	#[test]
	fn test_hash_u8() {
		assert_eq!(hash_u8(b"apple"), 1819308129);
	}
}
