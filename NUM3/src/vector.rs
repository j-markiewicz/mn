use std::{
	any,
	fmt::{Debug, Display, Formatter, Result as FmtResult},
	iter,
	ops::{Index, IndexMut},
};

use num_traits::Num;

/// A vector
#[derive(Clone, PartialEq)]
pub struct Vector<E> {
	elements: Box<[E]>,
}

impl<E> Vector<E> {
	pub fn n(&self) -> usize {
		self.elements.len()
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
		write!(f, "[ ")?;

		for e in self.elements.iter() {
			write!(f, "{e} ")?;
		}

		write!(f, "]")
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
