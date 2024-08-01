fn uint_div_const(div: u64) -> u64 {
	(1 << 63) / div
}

fn fast_mod(num: u64, div_const: u64, modulo: u64) -> u64 {
	let mut result = (num as u128 - (num as u128 * div_const as u128 >> 63) * modulo as u128) as u64;
	if result > modulo {
		result -= modulo;
	}
	result
}

fn mod_pow(base: u64, exp: u32, modulus: u64) -> u64 {
	let div_const = uint_div_const(modulus);

	let mut result = 1;
	let mut base = fast_mod(base, div_const, modulus);
	let mut exp = exp;

	while exp > 0 {
		if exp & 1 == 1 {
			result = fast_mod(result * base, div_const, modulus);
		}
		base = fast_mod(base * base, div_const, modulus);
		exp >>= 1;
	}

	result
}

fn miller_rabin_check(n: u32) -> u32 {
	if n <= 1 {
		return 0;
	}
	if n <= 3 {
		return 1;
	}
	if n & 1 == 0 {
		return 0;
	}
	2
}

fn miller_rabin_trailing_zeros(n: u32) -> (u32, u32) {
	// Write (n - 1) as 2^r * d
	let mut d = n - 1;
	let r = d.trailing_zeros();
	d >>= r;
	(r, d)
}

fn miller_rabin_loop(n: u64, r: u32, d: u32) -> bool {
	// Witness loop
	for a in [2, 7, 61] {
		if a >= n {
			break;
		}
		let mut x = mod_pow(a, d, n);
		if x == 1 || x == n - 1 {
			continue;
		}
		let mut continue_outer = true;
		for _ in 0..(r - 1) {
			x = mod_pow(x, 2, n);
			if x == n - 1 {
				continue_outer = false;
				break;
			}
		}
		if continue_outer {
			return false;
		}
	}
	true
}

pub fn is_prime(n: u32) -> bool {
	if miller_rabin_check(n) != 2 {
		return miller_rabin_check(n) == 1;
	}
	let (r, d) = miller_rabin_trailing_zeros(n);
	miller_rabin_loop(n as u64, r, d)
}

pub fn next_prime(n: u32) -> u32 {
	let mut num = n + 1;
	while !is_prime(num) {
		num += 1;
	}
	num
}

pub fn prev_prime(n: u32) -> u32 {
	let mut num = n - 1;
	while !is_prime(num) {
		num -= 1;
	}
	num
}


#[cfg(test)]
mod tests {
	use super::*;
	use std::time::Instant;

	/// `cargo test test_is_prime_speed --release`
	#[test]
	fn test_is_prime_speed() {
		let start = Instant::now();

		for i in 0..2u32.pow(26) {
			let result = is_prime(i);
			// println!("{} == prime = {}", i, result);
		}

		println!("Time elapsed for is_prime is: {:?}", start.elapsed());
	}

	#[test]
	fn test_small_primes() {
		assert!(is_prime(2));
		assert!(is_prime(3));
		assert!(is_prime(5));
		assert!(is_prime(7));
		assert!(is_prime(11));
	}

	#[test]
	fn test_small_composites() {
		assert!(!is_prime(1));
		assert!(!is_prime(4));
		assert!(!is_prime(6));
		assert!(!is_prime(8));
		assert!(!is_prime(9));
	}

	#[test]
	fn test_large_prime() {
		let large_prime = 4294967291;
		assert!(is_prime(large_prime));
	}

	#[test]
	fn test_large_composite() {
		let large_composite = 4294967295;
		assert!(!is_prime(large_composite));
	}

	#[test]
	fn test_next_prime() {
		assert_eq!(next_prime(2), 3);
		assert_eq!(next_prime(3), 5);
		assert_eq!(next_prime(4), 5);
		assert_eq!(next_prime(10), 11);
		assert_eq!(next_prime(20), 23);
	}

	#[test]
	fn test_prev_prime() {
		assert_eq!(prev_prime(3), 2);
		assert_eq!(prev_prime(5), 3);
		assert_eq!(prev_prime(6), 5);
		assert_eq!(prev_prime(12), 11);
		assert_eq!(prev_prime(25), 23);
	}
}
