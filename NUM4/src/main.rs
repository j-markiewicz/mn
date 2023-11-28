use nalgebra::LU;
use num4::{gen_b, gen_b_nalgebra, gen_matrix, gen_matrix_nalgebra, solve};

fn main() {
	let mat = gen_matrix(10);
	let b = gen_b(10);
	println!("A'₁₀ = \n{mat:4.1}");
	println!("b₁₀ = {b:.1}");
	let x = solve(mat, b);
	println!("x₁₀ = {x:.5}");

	let xnal = LU::new(gen_matrix_nalgebra(10))
		.solve(&gen_b_nalgebra(10))
		.unwrap();

	for i in 1..=10 {
		assert!((x[i] - xnal[i - 1]).abs() < f64::EPSILON);
	}

	let nal = gen_matrix_nalgebra(10);
	println!("Aₙₐₗ ₁₀ = {nal:.1}");

	let mat = gen_matrix(80);
	let nal = gen_matrix_nalgebra(80);

	let b = gen_b(80);
	let bnal = gen_b_nalgebra(80);

	for i in 1..=80 {
		assert!((b[i] - bnal[i - 1]).abs() < f64::EPSILON);
	}

	let x = solve(mat, b);
	let xnal = LU::new(nal).solve(&bnal).unwrap();

	for i in 1..=80 {
		assert!((x[i] - xnal[i - 1]).abs() < f64::EPSILON * 80.0);
	}

	println!("x₈₀ = {x:.8}");
}
