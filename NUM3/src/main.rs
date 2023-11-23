use nalgebra::LU;
use num3::{gen_matrix, gen_matrix_nalgebra, gen_vector, gen_vector_nalgebra};

fn main() {
	let mat = gen_matrix(10);
	println!("A₁₀ = \n{mat:.5}");
	let mat = mat.lu_decompose();
	println!("L͡U₁₀ = \n{mat:.5}");
	println!("det A₁₀ = {:.5}", mat.det());

	let x = gen_vector(10);
	let y = mat.solve(&x);

	println!("y₁₀ = {y}");

	let mat = gen_matrix(124);
	let mat = mat.lu_decompose();
	println!("det A₁₂₄ = {:.15}", mat.det());

	assert_eq!(
		gen_matrix(124).lu_decompose(),
		gen_matrix(124).lu_decompose_slow()
	);

	let nal = gen_matrix_nalgebra(124);
	let nal = LU::new(nal);
	assert!((mat.det() - nal.determinant()).abs() < f64::EPSILON);

	for i in 1..=124 {
		for j in i..=124 {
			assert!((mat[(i, j)] - nal.u()[(i - 1, j - 1)]).abs() < f64::EPSILON);
		}

		for j in 1..i {
			assert!((mat[(i, j)] - nal.l()[(i - 1, j - 1)]).abs() < f64::EPSILON);
		}
	}

	let x = gen_vector(124);
	let xnal = gen_vector_nalgebra(124);

	for i in 1..=124 {
		assert!((x[i] - xnal[i - 1]).abs() < f64::EPSILON);
	}

	let y = mat.solve(&x);
	let ynal = nal.solve(&xnal).unwrap();

	for i in 1..=124 {
		assert!((y[i] - ynal[i - 1]).abs() < f64::EPSILON * 124.0);
	}

	println!("y₁₂₄ = {y}");
}
