use std::ops::{Index, IndexMut};
use std::fmt::Debug;


pub struct TwoDVector<T> {
	data: Vec<Vec<T>>,
	inner_size: usize
}

impl<T: Debug> TwoDVector<T> {
	/// Creates a new instance with the given data and inner size.
	///
	/// # Panics
	/// * If the `inner_size` is zero.
	/// * If the input vector is empty.
	/// * If all rows don't have the same length.
	pub fn new(data: Vec<Vec<T>>, inner_size: usize) -> Self {
		if inner_size == 0 {
			panic!("TwoDVector::new() | inner_size {} | Specify inner_size.\n", inner_size);
		}

		let mut err_index = None;
		if data.iter().any(|row| {
			let len = row.len();
			if len > inner_size {
				err_index = Some(data.iter().position(|r| r.len() == len).unwrap());
				true
			} else {
				false
			}
		}) {
			match err_index {
				Some(i) => panic!(
					// It's the caller's job to fix the problem instead of bandaging it
					"TwoDVector::new() | data[{}] {:?} | inner_size {} | Row at index {} has more elements than allowed ({}).\n",
					i, data[i], inner_size, i, inner_size
				),
				None => unreachable!(), // This should never happen since we set err_index when finding an error
			}
		}

		TwoDVector { data, inner_size }
	}

	/// Immutable access to grid element at specified (x, y) coordinates.
	/// # Safety
	/// * The caller must ensure that `x` and `y` are valid indices within the bounds of `self.data`.
	pub fn xy(&self, x: usize, y: usize) -> &T {
		unsafe { self.data.get_unchecked(x).get_unchecked(y) }
	}

	/// Mutable access to grid element at specified (x, y) coordinates.
	/// # Safety
	/// * The caller must ensure that `x` and `y` are valid indices within the bounds of `self.data`.
	pub fn xy_mut(&mut self, x: usize, y: usize) -> &mut T {
		unsafe { self.data.get_unchecked_mut(x).get_unchecked_mut(y) }
	}
}

impl<T> Index<usize> for TwoDVector<T> {
	type Output = T;

	fn index(&self, index: usize) -> &T {
		let x = index / self.inner_size; let y = index % self.inner_size;
		&self.data[x][y]
	}
}

impl<T> IndexMut<usize> for TwoDVector<T> {
	fn index_mut(&mut self, index: usize) -> &mut T {
		let x = index / self.inner_size; let y = index % self.inner_size;
		&mut self.data[x][y]
	}
}


#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_two_d_vector() {
		let data = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
		let mut vec = TwoDVector::new(data, 3);

		*vec.xy_mut(2, 1) += 2;
		vec[7] += 2;
		assert_eq!(*vec.xy(2, 1), 12);
	}

	#[test]
	#[should_panic]
	fn test_zero_size() {
		TwoDVector::new(vec![Vec::<i32>::new()], 0);
	}

	#[test]
	#[should_panic]
	fn test_bigger_than_inner_size() {
		let data = vec![vec![1, 2, 3], vec![4, 5, 6, 7], vec![7, 8]];
		TwoDVector::new(data, 3);
	}
}
