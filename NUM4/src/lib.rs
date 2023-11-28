mod matrix;
mod vector;

use std::{iter, ops::Add};

use matrix::BandMatrix;
use nalgebra::{DMatrix, DVector};
use num_traits::Num;
use vector::Vector;

/// Solve $(A' + uv^T)x = b$ for $x$, where $u = v = [ 1 1 ... 1 ]^T$
pub fn solve<E: Num + Copy, const L: usize, const U: usize>(
	a_prime: BandMatrix<E, L, U>,
	b: Vector<E>,
) -> Vector<E> {
	assert_eq!(a_prime.n(), b.n(), "A''s height must equal b's size");

	// $A' = LU$
	let lu = a_prime.lu_decompose();

	// $LUy = b$
	let y = lu.solve(&b);

	// $LUz = u$
	let z = lu.solve(&Vector::from_iter(iter::repeat(E::one()).take(b.n())));

	// $zv^Ty = (\sum_{i=1}^n y_i)z$
	let vy = y.iter().copied().reduce(Add::add).unwrap();

	// $v^Tz = \sum_{i=1}^n z_i$
	let vz = z.iter().copied().reduce(Add::add).unwrap();

	// $x = y - \frac{zv^Ty}{1 + v^Tz}$
	y - Vector::from_iter(z.into_iter().map(|z| z * vy)) / (E::one() + vz)
}

/// Generate the band matrix A' from NUM4
pub fn gen_matrix(n: usize) -> BandMatrix<f64, 0, 1> {
	let mut res = BandMatrix::new(n);

	for i in 1..=n {
		res[(i, i)] = 12.0 - 1.0;
	}

	for i in 1..=n - 1 {
		res[(i, i + 1)] = 8.0 - 1.0;
	}

	res
}

/// Generate the vector u = vᵀ from NUM4
pub fn gen_u(n: usize) -> Vector<f64> {
	Vector::from_iter(iter::repeat(1.0).take(n))
}

/// Generate the vector b from NUM4
pub fn gen_b(n: usize) -> Vector<f64> {
	Vector::from_iter(iter::repeat(5.0).take(n))
}

/// Generate the matrix A from NUM4
pub fn gen_matrix_nalgebra(n: usize) -> DMatrix<f64> {
	let mut res = DMatrix::zeros(n, n);
	res.fill(1.0);

	for i in 0..n {
		res[(i, i)] = 12.0;
	}

	for i in 0..n - 1 {
		res[(i, i + 1)] = 8.0;
	}

	res
}

/// Generate the vector b from NUM4
pub fn gen_b_nalgebra(n: usize) -> DVector<f64> {
	DVector::from(iter::repeat(5.0).take(n).collect::<Vec<_>>())
}
