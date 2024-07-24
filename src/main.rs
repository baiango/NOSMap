mod two_d_vector;
use two_d_vector::TwoDVector;


fn main() {
	let data = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
	let vec = TwoDVector::new(data, 3);
}
