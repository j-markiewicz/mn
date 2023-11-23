mod matrix;
mod vector;

use matrix::BandMatrix;
use nalgebra::{DMatrix, DVector};
use vector::Vector;

/// Generate the matrix from NUM3
pub fn gen_matrix(n: usize) -> BandMatrix<f64, 1, 2> {
	let mut res = BandMatrix::new(n);

	for i in 1..=n - 1 {
		res[(i + 1, i)] = 0.2;
	}

	for i in 1..=n {
		res[(i, i)] = 1.2;
	}

	for i in 1..=n - 1 {
		res[(i, i + 1)] = 0.1 / i as f64;
	}

	for i in 1..=n - 2 {
		res[(i, i + 2)] = 0.15 / (i as f64).powi(2);
	}

	res
}

/// Generate the vector from NUM3
pub fn gen_vector(n: usize) -> Vector<f64> {
	Vector::from_iter((1..=n).map(|x| x as f64))
}

/// Generate the matrix from NUM3
pub fn gen_matrix_nalgebra(n: usize) -> DMatrix<f64> {
	let mut res = DMatrix::zeros(n, n);

	for i in 0..n - 1 {
		res[(i + 1, i)] = 0.2;
	}

	for i in 0..n {
		res[(i, i)] = 1.2;
	}

	for i in 0..n - 1 {
		res[(i, i + 1)] = 0.1 / (i + 1) as f64;
	}

	for i in 0..n - 2 {
		res[(i, i + 2)] = 0.15 / ((i + 1) as f64).powi(2);
	}

	res
}

/// Generate the vector from NUM3
pub fn gen_vector_nalgebra(n: usize) -> DVector<f64> {
	DVector::from((1..=n).map(|x| x as f64).collect::<Vec<_>>())
}
