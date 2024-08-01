use std::simd::{u64x4, ToBytes};


pub fn sum_u64x4_scalar(simds: u64x4) -> u64 {
	simds[0].wrapping_add(simds[1]).wrapping_add(simds[2]).wrapping_add(simds[3])
}

pub fn hash(input_data: &[u64x4]) -> u64 {
	let mut hash = u64x4::splat(0);

	for i in 0..input_data.len() {
		hash += input_data[i];
	}

	sum_u64x4_scalar(hash)
}

/// Recommend to use `hash(&[u64x4])` instead of `hash_u8(&[u8])`.
/// Because `hash_u8(&[u8])` is bottlenecked by all 16 x64 registers.
pub fn hash_u8(input_data: &[u8]) -> u64 {
	let mut hash = u64x4::splat(0);

	for chunk in input_data.chunks(32) {
		let mut data_arr = [0u8; 32];
		data_arr[..chunk.len()].copy_from_slice(chunk);
		let data = u64x4::from_le_bytes(data_arr.into());

		hash += data;
	}

	sum_u64x4_scalar(hash)
}


#[cfg(test)]
mod tests {
	use crate::vasthash_b::*;

	#[test]
	fn test_hash() {
		let result = hash(&vec![u64x4::splat(123), u64x4::splat(123)]);
		assert_eq!(result, 984);
	}

	#[test]
	fn test_hash_u8() {
		assert_eq!(hash_u8(b"apple"), 435611005025);
	}
}
