mod matrix;
mod vector;

pub use matrix::BandMatrix;
pub use vector::Vector;

/// Generate the band matrix A from NUM5
pub fn gen_matrix(n: usize) -> BandMatrix<f64, 2, 2> {
	let mut res = BandMatrix::new(n);

	for i in 1..=n {
		res[(i, i)] = 3.0;
	}

	for i in 1..=n - 1 {
		res[(i, i + 1)] = 1.0;
	}

	for i in 1..=n - 2 {
		res[(i, i + 2)] = 0.15;
	}

	for i in 1..=n - 1 {
		res[(i + 1, i)] = 1.0;
	}

	for i in 1..=n - 2 {
		res[(i + 2, i)] = 0.15;
	}

	res
}

/// Generate the vector b from NUM5
pub fn gen_b(n: usize) -> Vector<f64> {
	Vector::from_iter((1..=n).map(|i| i as f64))
}
