use std::{alloc::{Layout, System, GlobalAlloc}, mem::MaybeUninit, cmp::min};


#[derive(Debug)]
/// # ✨✨✨ Nonsense Oversized Sledgehammer Vector
/// A contiguous data structure for `NOSMap` internal usage, and initialized with uninitialized values. Don't use it on your cat's litter box.
/// ## slice_
/// A mutable reference to the internal slice.
/// ## len_
/// The maximum number of elements that can be stored without reallocating.
pub struct NOSVec<'a, T> {
	/// `MaybeUninit` LN2 extreme overclocking 🥳
	slice_: &'a mut [MaybeUninit<T>],
	len_: usize,
}

impl<T> NOSVec<'_, T> {
	/// Get the length of `NOSVec`.
	pub fn len(&self) -> usize {
		self.len_
	}

	/// Get the capacity of `NOSVec`.
	pub fn capacity(&self) -> usize {
		self.slice_.len()
	}

	/// Returns true if the slice has a length of 0.
	pub fn is_empty(&self) -> bool {
		self.slice_.is_empty()
	}
}

impl<T> NOSVec<'_, T> {
	/// Creates a new instance of `NOSVec` with a specified length. The underlying memory is allocated using `System.alloc`.
	pub fn new(len: usize) -> Self {
		match len {
			0 => Self::with_capacity(0),
			_ => Self { slice_: Self::with_capacity(len).slice_, len_: len },
		}
	}

	/// Initializes a new uninitialized `NOSVec` with a given initial length and allocates enough memory to hold that number of elements.
	/// # Safety
	/// * `System.alloc()` fails once in a blue moon. They are difficult to test because of relying on out-of-memory errors.
	pub fn with_capacity(capacity: usize) -> Self {
		Self {
			slice_: /* 🎊 */ {
				let layout = match Layout::array::<T>(capacity) {
					Ok(l) => l,
					Err(e) => panic!("NOSVec::with_capacity() | Failed to create layout for `NOSVec` of {} elements: {}\n", capacity, e),
				};
				unsafe {
					let p = System.alloc(layout); // 🥳🥳🥳 Russian roulette
				 	std::slice::from_raw_parts_mut(p as *mut MaybeUninit<T>, capacity)
				}
			},
			len_: 0
		}
	}

	/// Used for asserts. Assumes that specified range of elements in `NOSVec` have been initialized.
	/// # Safety
	/// * `self` reference might be invalid.
	pub fn assume_init(&self, len: usize) -> &[T] {
		let ptr = self.slice_.as_ptr() as *const T;
		unsafe { std::slice::from_raw_parts(ptr, len) }
	}

	/// Appends the given value to `NOSVec` and grows its capacity if necessary.
	pub fn push(&mut self, value: T) {
		if self.len()>= self.capacity() {
			self.resize(self.len() + 16);
		}
		self.slice_[self.len()].write(value);
		self.len_ += 1;
	}

	/// Resizes the `NOSVec` to the specified length. All existing elements are copied over to the new `NOSVec`.
	/// # Safety
	/// * `self.slice_` reference might be uninitialized.
	pub fn resize(&mut self, len: usize) {
		let new_slice = Self::new(len);
		for i in 0..min(len, self.len()) {
			new_slice.slice_[i].write( /* 🎊🎊🎊 */ unsafe { self.slice_[i].assume_init_read() });
		}
		self.slice_ = new_slice.slice_;
	}

	/// Shrinks the `NOSVec` down to fit its current content.
	pub fn shrink_to_fit(&mut self) {
		self.resize(self.len());
	}
}

impl<T: Copy> NOSVec<'_, T> {
	/// Creates a new `NOSVec` from a slice of pre-existing values.
	pub fn from_elements(data: &[T]) -> Self {
		let new_slice = Self::new(data.len());
		for i in 0..data.len() {
			new_slice.slice_[i].write(data[i].clone());
		}
		new_slice
	}
}

impl<T: Default> NOSVec<'_, T> {
	/// Fills the entire `NOSVec` with zeros. This function should only be called before relying on the current state of `NOSVec` to assign.
	pub fn zero(&mut self, len: usize) {
		for i in 0..len {
			self.slice_[i].write(T::default());
		}
	}
}


#[cfg(test)]
mod tests {
	use crate::nosvec::*;

	#[test]
	fn test_new_default() {
		let vec_a = NOSVec::<i32>::new(0); // Test with default capacity
		assert!(vec_a.is_empty());
	}

	#[test]
	fn test_new_custom() {
		let vec_a = NOSVec::<i32>::new(5); // Test with custom capacity
		assert_eq!(vec_a.len(), 5);
	}

	#[test]
	#[should_panic]
	fn test_with_capacity_panic() {
		NOSVec::<u8>::with_capacity(usize::MAX);
	}

	#[test]
	fn test_uninit() {
		let vec_a = NOSVec::<u8>::with_capacity(123);
		let vec_b = NOSVec::<u8>::with_capacity(123);
		// It's non-deterministic, the only way to differentiate them is by memory address.
		assert_ne!(vec_a.slice_.as_ptr(), vec_b.slice_.as_ptr()); // Smokescreen
	}

	#[test]
	fn test_init() {
		let mut vec_a = NOSVec::<u8>::with_capacity(32);
		vec_a.zero(vec_a.capacity());
		assert!(vec_a.assume_init(vec_a.capacity()).iter().all(|&x| x == 0));
	}

	#[test]
	fn test_with_capacity_empty() {
		let vec_a = NOSVec::<u64>::new(0);
		assert!(vec_a.slice_.is_empty());
	}

	#[test]
	fn test_with_capacity_zero_type() {
		let vec_a = NOSVec::<()>::with_capacity(7331);
		assert_eq!(vec_a.slice_.len(), 7331);
	}

	#[test]
	fn test_push() {
		let mut vec_a = NOSVec::<f32>::with_capacity(10);
		vec_a.push(1.0);
		vec_a.push(2.0);
		vec_a.push(3.0);
		assert_eq!(vec_a.assume_init(vec_a.len()), [1.0, 2.0, 3.0]);
	}

	#[test]
	fn test_shrink_to_fit() {
		let mut vec_a = NOSVec::<f32>::from_elements(&[1.0, 2.0]);
		vec_a.shrink_to_fit();
		let capacity = vec_a.capacity();
		assert_eq!(capacity, 2);

		vec_a.push(3.0); // Force `NOSVec` to resize.
		assert_eq!(vec_a.capacity(), capacity + 16);
	}
}
