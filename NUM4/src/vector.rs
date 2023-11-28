use core::slice;
use std::{
	any,
	fmt::{Debug, Display, Formatter, Result as FmtResult},
	iter,
	ops::{Div, Index, IndexMut, Sub},
	vec,
};

use num_traits::Num;

/// A column vector
#[derive(Clone, PartialEq)]
pub struct Vector<E> {
	elements: Box<[E]>,
}

impl<E> Vector<E> {
	pub fn n(&self) -> usize {
		self.elements.len()
	}

	pub fn iter(&self) -> impl Iterator<Item = &E> {
		self.into_iter()
	}
}

impl<E: Num> Vector<E> {
	/// Create a new zeroed vector of length n
	pub fn new(n: usize) -> Self {
		Self::from_iter(iter::repeat_with(E::zero).take(n))
	}
}

impl<E> FromIterator<E> for Vector<E> {
	fn from_iter<T: IntoIterator<Item = E>>(iter: T) -> Self {
		Self {
			elements: iter.into_iter().collect(),
		}
	}
}

impl<E: Debug> Debug for Vector<E> {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		if f.alternate() {
			f.debug_struct(&format!("Vector<{}>", any::type_name::<E>()))
				.field("elements", &self.elements)
				.finish()
		} else {
			f.debug_list().entries(self.elements.iter()).finish()
		}
	}
}

impl<E: Display> Display for Vector<E> {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		let width = f.width().unwrap_or(1);
		let precision = f.precision().unwrap_or(6);

		write!(f, "[ ")?;

		for e in self.elements.iter() {
			write!(
				f,
				"{e:width$.precision$} ",
				width = width,
				precision = precision
			)?;
		}

		write!(f, "]ᵀ")
	}
}

impl<E> Index<usize> for Vector<E> {
	type Output = E;

	fn index(&self, index: usize) -> &Self::Output {
		assert_ne!(
			index, 0,
			"Index out of bounds: Vectors use 1-based indexing"
		);

		&self.elements[index - 1]
	}
}

impl<E> IndexMut<usize> for Vector<E> {
	fn index_mut(&mut self, index: usize) -> &mut Self::Output {
		assert_ne!(
			index, 0,
			"Index out of bounds: Vectors use 1-based indexing"
		);

		&mut self.elements[index - 1]
	}
}

impl<'a, E> IntoIterator for &'a Vector<E> {
	type Item = &'a E;
	type IntoIter = slice::Iter<'a, E>;

	fn into_iter(self) -> Self::IntoIter {
		self.elements.iter()
	}
}

impl<E> IntoIterator for Vector<E> {
	type Item = E;
	type IntoIter = vec::IntoIter<E>;

	fn into_iter(self) -> Self::IntoIter {
		self.elements.into_vec().into_iter()
	}
}

impl<E: Num + Copy> Div<E> for Vector<E> {
	type Output = Vector<E>;

	fn div(mut self, rhs: E) -> Self::Output {
		for e in self.elements.iter_mut() {
			*e = *e / rhs;
		}

		self
	}
}

impl<E: Num + Copy> Sub<Self> for Vector<E> {
	type Output = Vector<E>;

	fn sub(mut self, rhs: Self) -> Self::Output {
		for (a, b) in self.elements.iter_mut().zip(rhs.into_iter()) {
			*a = *a - b;
		}

		self
	}
}
