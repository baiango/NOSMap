use std::{alloc::{Layout, System, GlobalAlloc}, mem::MaybeUninit, ptr, cmp::min};


#[derive(Debug)]
/// # ✨✨✨ Nonsense vector - implemented with a sledgehammer
/// A contiguous data structure for `NOSMap` internal usage, and initialized with uninitialized values.
/// ## slice_
/// A mutable reference to the internal slice.
/// ## len_
/// The maximum number of elements that can be stored without reallocating.
pub struct NOSVec<'a, T> {
	/// `&'a mut [MaybeUninit<T>]` managed to reduce excess instructions by 1 than `&'a mut [T]` on `new()`.
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
}

impl<T> NOSVec<'_, T> {
	/// Creates a new empty `NOSVec`. The underlying memory is allocated using `System.alloc`.
	/// # Safety
	/// * `unwrap_unchecked()` and `System.alloc()` fails once in a blue moon. They are difficult to test because of relying on out-of-memory errors.
	pub fn new() -> Self {
		Self::with_capacity(0)
	}

	/// Creates a new instance of `NOSVec` with a specified length and default capacity.
	/// # Safety
	/// * `unwrap_unchecked()` and `System.alloc()` fails once in a blue moon. They are difficult to test because of relying on out-of-memory errors.
	pub fn with_length(len: usize) -> Self {
		Self { slice_: Self::with_capacity(len).slice_, len_: len }
	}

	/// Initializes a new uninitialized `NOSVec` with a given initial length and allocates enough memory to hold that number of elements.
	/// # Safety
	/// * `unwrap_unchecked()` and `System.alloc()` fails once in a blue moon. They are difficult to test because of relying on out-of-memory errors.
	pub fn with_capacity(len: usize) -> Self {
		let layout = unsafe { Layout::array::<T>(len).unwrap_unchecked() }; // Russian roulette
		Self {
			slice_: unsafe {
				let p = System.alloc(layout);
				std::slice::from_raw_parts_mut(p as *mut MaybeUninit<T>, len)
			},
			len_: 0
		}
	}

	// Used for asserts. Assumes that all elements in `NOSVec` have been initialized.
	pub fn slice_assume_init(&mut self) -> &[T] {
		unsafe {
			let ptr = self.slice_.as_mut_ptr() as *mut T;
			std::slice::from_raw_parts(ptr, self.len())
		}
	}

	/// Appends the given value to `NOSVec` and grows its capacity if necessary.
	pub fn push(&mut self, value: T) {
		if self.len() >= self.capacity() {
			self.resize(self.len() + 16);
		}
		self.slice_[self.len()].write(value);
		self.len_ += 1;
	}

	/// Resizes the `NOSVec` to the specified length. All existing elements are copied over to the new `NOSVec`.
	/// # Safety
	/// * `unwrap_unchecked()` and `System.alloc()` fails once in a blue moon. They are difficult to test because of relying on out-of-memory errors.
	pub fn resize(&mut self, len: usize) {
		let new_slice = Self::with_capacity(len);
		for i in 0..min(len, self.len()) {
			new_slice.slice_[i].write(unsafe { self.slice_[i].assume_init_read() });
		}
		self.slice_ = new_slice.slice_;
	}
}

impl<T: Copy> NOSVec<'_, T> {
	/// Shrinks the `NOSVec` down to fit its current content.
	/// # Safety
	/// * `unwrap_unchecked()` and `System.alloc()` fails once in a blue moon. They are difficult to test because of relying on out-of-memory errors.
	pub fn shrink_to_fit(&mut self) {
		let new_slice = Self::with_capacity(self.len());
		for i in 0..self.len() {
			new_slice.slice_[i].write(unsafe { self.slice_[i].assume_init_read() });
		}
		self.slice_ = new_slice.slice_;
	}

	/// Creates a new `NOSVec` from a slice of pre-existing values.
	pub fn from_elements(data: &[T]) -> Self {
		let new_slice = Self::with_length(data.len());
		for i in 0..data.len() {
			new_slice.slice_[i].write(data[i].clone());
		}
		new_slice
	}
}

impl<T: Default> NOSVec<'_, T> {
	/// Fills the entire vector with zeros. This function should only be called before relying on the current state of `NOSVec` to assign.
	pub fn zero(&mut self) {
		self.slice_.iter_mut().for_each(|x| { x.write(T::default()); });
	}
}


#[cfg(test)]
mod tests {
	use crate::nosvec::*;

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
		vec_a.zero();
		assert!(unsafe { vec_a.slice_.iter().all(|x| x.assume_init() == 0)} );
	}

	#[test]
	fn test_with_capacity_empty() {
		let vec_a = NOSVec::<u64>::new();
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
		assert_eq!(vec_a.slice_assume_init(), [1.0, 2.0, 3.0]);
	}

	#[test]
	fn test_shrink_to_fit() {
		let mut vec_a = NOSVec::<f32>::from_elements(&[1.0, 2.0]);
		vec_a.shrink_to_fit();
		assert_eq!(vec_a.capacity(), 2);

		vec_a.push(3.0); // Force `NOSVec` to resize.
	}
}
