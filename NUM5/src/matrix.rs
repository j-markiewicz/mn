use std::{
	any,
	fmt::{Debug, Display, Formatter, Result as FmtResult},
	iter,
	ops::{Add, Index, IndexMut},
};

use num_traits::{real::Real, Num};

use crate::vector::Vector;

/// A square [Band Matrix](https://en.wikipedia.org/wiki/Band_matrix) with L
/// elements below/left of the main diagonal and U elements above/right of the
/// main diagonal. Requires only `O(n)` storage space. Indexing into the matrix
/// works as usual, but with the restriction that all elements outside of the
/// bands are all 0 and read-only (mutable indexing on them will panic).
#[derive(Clone)]
pub struct BandMatrix<E, const L: usize, const U: usize> {
	n: usize,
	zero: E,
	elements: Box<[E]>,
}

/// The LU factors of a square Band Matrix with L elements below/left of the
/// main diagonal and U elements above/right of the main diagonal.
#[derive(Clone, PartialEq)]
pub struct LuMatrices<E, const L: usize, const U: usize>(BandMatrix<E, L, U>, E);

impl<E, const L: usize, const U: usize> BandMatrix<E, L, U> {
	pub fn n(&self) -> usize {
		self.n
	}

	fn idx(&self, index: (usize, usize)) -> Option<usize> {
		assert_ne!(
			index.0, 0,
			"Index {index:?} out of bounds: matrices use 1-based indexing"
		);
		assert_ne!(
			index.1, 0,
			"Index {index:?} out of bounds: matrices use 1-based indexing"
		);

		assert!(
			index.0 <= self.n,
			"Index {index:?} out of bounds: column index is {} but matrix is {}x{}",
			index.0,
			self.n,
			self.n,
		);
		assert!(
			index.1 <= self.n,
			"Index {index:?} out of bounds: row index is {} but matrix is {}x{}",
			index.1,
			self.n,
			self.n
		);

		let i = index.0 - 1;
		let j = index.1 - 1;

		if i < j && j - i > U {
			return None;
		} else if i > j && i - j > L {
			return None;
		}

		let idx = if i < j {
			(L + U + 1) * i + L + j.abs_diff(i)
		} else {
			(L + U + 1) * i + L - j.abs_diff(i)
		};

		Some(idx)
	}
}

impl<E: Num, const L: usize, const U: usize> BandMatrix<E, L, U> {
	/// Create a new zeroed Band Matrix of size n by n
	pub fn new(n: usize) -> Self {
		Self {
			n,
			zero: E::zero(),
			elements: iter::repeat_with(E::zero).take((U + L + 1) * n).collect(),
		}
	}
}

impl<E: Real + Copy, const L: usize, const U: usize> BandMatrix<E, L, U> {
	/// Perform the Gauss-Seidel algorithm, returning the result if it converged
	pub fn gauss_seidel(
		&self,
		mut x: Vector<E>,
		b: &Vector<E>,
		epsilon: E,
		max_iters: usize,
	) -> Option<Vector<E>> {
		assert_eq!(self.n(), x.n());
		assert_eq!(x.n(), b.n());

		let mut next;

		for _ in 0..max_iters {
			next = self.gauss_seidel_iteration(&x, b);

			if (&x - &next).norm() < epsilon {
				return Some(next);
			}

			x = next;
		}

		None
	}

	/// Perform the Jacobi algorithm, returning the result if it converged
	pub fn jacobi(
		&self,
		mut x: Vector<E>,
		b: &Vector<E>,
		epsilon: E,
		max_iters: usize,
	) -> Option<Vector<E>> {
		assert_eq!(self.n(), x.n());
		assert_eq!(x.n(), b.n());

		let mut next;

		for _ in 0..max_iters {
			next = self.jacobi_iteration(&x, b);

			if (&x - &next).norm() < epsilon {
				return Some(next);
			}

			x = next;
		}

		None
	}
}

impl<E: Num + Copy, const L: usize, const U: usize> BandMatrix<E, L, U> {
	/// Perform one Gauss-Seidel iteration, returning the next `x`
	pub fn gauss_seidel_iteration(&self, x: &Vector<E>, b: &Vector<E>) -> Vector<E> {
		assert_eq!(self.n(), x.n());
		assert_eq!(x.n(), b.n());

		let mut next = Vector::new(x.n());

		for i in 1..=x.n() {
			// $x_i^{(k+1)} = \frac{1}{a_{i,i}} (b_i - \sum_{j = 1}^{i - 1} a_{i,j}x_j^{(k + 1)} - \sum_{j = i + 1}^n a_{i,j}x_j^{(k)})$
			next[i] = (b[i]
				- (1..=i - 1)
					.map(|j| self[(i, j)] * next[j])
					.reduce(Add::add)
					.unwrap_or(E::zero())
				- (i + 1..=x.n())
					.map(|j| self[(i, j)] * x[j])
					.reduce(Add::add)
					.unwrap_or(E::zero()))
				/ (self[(i, i)]);
		}

		next
	}

	/// Perform one Jacobi iteration, returning the next `x`
	pub fn jacobi_iteration(&self, x: &Vector<E>, b: &Vector<E>) -> Vector<E> {
		assert_eq!(self.n(), x.n());
		assert_eq!(x.n(), b.n());

		let mut next = Vector::new(x.n());

		for i in 1..=x.n() {
			// $x_i^{(k+1)} = \frac{1}{a_{i,i}} (b_i - \sum_{j \ne i} a_{i,j}x_j^{(k)})$
			next[i] = (b[i]
				- (1..=x.n())
					.filter(|&j| j != i)
					.map(|j| self[(i, j)] * x[j])
					.reduce(Add::add)
					.unwrap_or(E::zero()))
				/ (self[(i, i)]);
		}

		next
	}

	/// Perform in-place LU decomposition of this matrix. This method assumes
	/// that the zero element behaves like the number 0, i.e. `x + 0 = x` and
	/// `x * 0 = 0`, and that the matrix admits LU decomposition. After this
	/// method completes, this matrix will contain the L and U matrices, with
	/// the L matrix's diagonal elements implied to be 1 and not stored.
	pub fn lu_decompose(mut self) -> LuMatrices<E, L, U> {
		if self.n <= 1 {
			return LuMatrices(self, E::one());
		}

		// Based on <http://mathonline.wikidot.com/the-algorithm-for-doolittle-s-method-for-lu-decompositions>,
		// but optimized to be efficient for band matrices
		for k in 1..=self.n {
			// $u_{k,m} = a_{k,m} - \sum_{j=\max(1, k - L, m - U)}^{k-1} l_{k,j} u_{j,m}$ for $m = k, k + 1, ..., k + U \le n$
			for m in k..=(k + U).min(self.n) {
				self[(k, m)] = self[(k, m)]
					- (1.max(k.saturating_sub(L)).max(m.saturating_sub(U))..=k - 1)
						.map(|j| self[(k, j)] * self[(j, m)])
						.reduce(Add::add)
						.unwrap_or_else(E::zero);
			}

			// $l_{i,k} = \frac{(a_{i,k} - \sum_{j=\max(1, i - L, k - U)}^{k-1} l_{i,j} u_{j,k})}{u_{k,k}}$ for $i = k + 1, k + 2, ..., k + L \le n$
			for i in k + 1..=(k + L).min(self.n) {
				self[(i, k)] = (self[(i, k)]
					- (1.max(i.saturating_sub(L)).max(k.saturating_sub(U))..=k - 1)
						.map(|j| self[(i, j)] * self[(j, k)])
						.reduce(Add::add)
						.unwrap_or_else(E::zero))
					/ self[(k, k)];
			}
		}

		LuMatrices(self, E::one())
	}
}

impl<E, const L: usize, const U: usize> LuMatrices<E, L, U> {
	/// Get the `(i, j)`th index of the lower matrix
	pub fn l(&self, i: usize, j: usize) -> &E {
		if i == j {
			&self.1
		} else if i < j {
			&self.0.zero
		} else {
			&self[(i, j)]
		}
	}

	/// Get the `(i, j)`th index of the upper matrix
	pub fn u(&self, i: usize, j: usize) -> &E {
		if i > j {
			&self.0.zero
		} else {
			&self[(i, j)]
		}
	}
}

impl<E: Num + Copy, const L: usize, const U: usize> LuMatrices<E, L, U> {
	pub fn det(&self) -> E {
		let mut res = E::one();

		for i in 1..=self.0.n {
			res = res * self[(i, i)];
		}

		res
	}

	pub fn solve(&self, b: &Vector<E>) -> Vector<E> {
		assert_eq!(
			self.0.n(),
			b.n(),
			"Can't solve system of equations for a matrix and vector with different heights"
		);

		let mut y = Vector::new(b.n());
		let mut x = Vector::new(b.n());

		// $Ly = b$
		// $y_m = \frac{b_m - \sum_{i=1}^{m-1} l_{m,i} y_i}{l_{m,m}}$ for $m = 1, ..., n$
		// $= b_m - \sum_{i=\max(1, m - L)}^{m-1} l_{m,i} y_i$ for $m = 1, ..., n$
		for m in 1..=self.0.n() {
			y[m] = b[m]
				- (1.max(m.saturating_sub(L))..=m - 1)
					.map(|i| *self.l(m, i) * y[i])
					.reduce(Add::add)
					.unwrap_or_else(E::zero);
		}

		// $Ux = y$
		// $x_m = \frac{y_m - \sum_{i=m+1}^{n} u_{m,i} x_i}{u_{m,m}}$ for $m = n, ..., 1$
		// $= \frac{y_m - \sum_{i=m+1}^{\min(n, m + U)} u_{m,i} x_i}{u_{m,m}}$ for $m = n, ..., 1$
		for m in (1..=self.0.n()).rev() {
			x[m] = (y[m]
				- (m + 1..=self.0.n().min(m + U))
					.map(|i| *self.u(m, i) * x[i])
					.reduce(Add::add)
					.unwrap_or_else(E::zero))
				/ *self.u(m, m);
		}

		x
	}
}

impl<E: PartialEq, const L: usize, const U: usize> PartialEq for BandMatrix<E, L, U> {
	fn eq(&self, other: &Self) -> bool {
		self.elements == other.elements
	}
}

impl<E: Debug, const L: usize, const U: usize> Debug for BandMatrix<E, L, U> {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		if f.alternate() {
			f.debug_struct(&format!(
				"BandMatrix<{}, L = {L}, U = {U}>",
				any::type_name::<E>()
			))
			.field("n", &self.n)
			.field("zero", &self.zero)
			.field("elements", &self.elements)
			.finish()
		} else {
			let width = f.width().unwrap_or(1);
			let precision = f.precision().unwrap_or(6);
			let full_width =
				(width * (L + U + 1) + L + U).max((2 + precision) * (L + U + 1) + L + U);

			write!(f, "┌ ")?;
			for _ in 0..full_width {
				write!(f, " ")?;
			}
			writeln!(f, " ┐(L = {L}, U = {U})")?;

			for i in 0..self.n {
				write!(f, "┆ ")?;

				for j in 0..(L + U + 1) {
					write!(
						f,
						"{:width$.precision$?} ",
						self.elements[(L + U + 1) * i + j],
						width = width,
						precision = precision
					)?;
				}

				writeln!(f, "┆")?;
			}

			write!(f, "└ ")?;
			for _ in 0..full_width {
				write!(f, " ")?;
			}
			writeln!(f, " ┘{}×{}", self.n, self.n)?;

			Ok(())
		}
	}
}

impl<E: Debug, const L: usize, const U: usize> Debug for LuMatrices<E, L, U> {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		self.0.fmt(f)
	}
}

impl<E: Display, const L: usize, const U: usize> Display for BandMatrix<E, L, U> {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		let width = f.width().unwrap_or(1);
		let precision = f.precision().unwrap_or(6);
		let full_width = (width * self.n + self.n - 1).max((2 + precision) * self.n + self.n - 1);

		write!(f, "┌ ")?;
		for _ in 0..full_width {
			write!(f, " ")?;
		}
		writeln!(f, " ┐")?;

		for i in 1..=self.n {
			write!(f, "│ ")?;

			for j in 1..=self.n {
				write!(
					f,
					"{:width$.precision$} ",
					self[(i, j)],
					width = width,
					precision = precision
				)?;
			}

			writeln!(f, "│")?;
		}

		write!(f, "└ ")?;
		for _ in 0..full_width {
			write!(f, " ")?;
		}
		writeln!(f, " ┘{}×{}", self.n, self.n)?;

		Ok(())
	}
}

impl<E: Display, const L: usize, const U: usize> Display for LuMatrices<E, L, U> {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		self.0.fmt(f)
	}
}

impl<E, const L: usize, const U: usize> Index<(usize, usize)> for BandMatrix<E, L, U> {
	type Output = E;

	fn index(&self, index: (usize, usize)) -> &Self::Output {
		if let Some(idx) = self.idx(index) {
			&self.elements[idx]
		} else {
			&self.zero
		}
	}
}

impl<E, const L: usize, const U: usize> IndexMut<(usize, usize)> for BandMatrix<E, L, U> {
	fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
		if let Some(idx) = self.idx(index) {
			&mut self.elements[idx]
		} else {
			panic!("Index {index:?} out of bounds for BandMatrix<_, {L}, {U}>({}): mutable indexing can not access zero elements", self.n)
		}
	}
}

impl<E, const L: usize, const U: usize> Index<(usize, usize)> for LuMatrices<E, L, U> {
	type Output = E;

	fn index(&self, index: (usize, usize)) -> &Self::Output {
		&self.0[index]
	}
}

impl<E, const L: usize, const U: usize> IndexMut<(usize, usize)> for LuMatrices<E, L, U> {
	fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
		&mut self.0[index]
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn decompose() {
		let mut a = BandMatrix::<f64, 1, 1>::new(2);

		a[(1, 1)] = 4.0;
		a[(1, 2)] = 3.0;
		a[(2, 1)] = 6.0;
		a[(2, 2)] = 3.0;

		let a = a.lu_decompose();

		assert_eq!(a[(1, 1)], 4.0);
		assert_eq!(a[(1, 2)], 3.0);
		assert_eq!(a[(2, 1)], 1.5);
		assert_eq!(a[(2, 2)], -1.5);
	}
}
