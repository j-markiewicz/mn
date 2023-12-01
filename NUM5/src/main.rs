use std::iter;

use plotters::prelude::*;

use num5::{gen_b, gen_matrix, BandMatrix, Vector};

fn superscript(n: usize) -> String {
	const DIGITS: &[char] = &['⁰', '¹', '²', '³', '⁴', '⁵', '⁶', '⁷', '⁸', '⁹'];

	n.to_string()
		.chars()
		.map(|c| c.to_digit(10).unwrap())
		.map(|d| DIGITS[d as usize])
		.collect()
}

fn main() {
	let mat = gen_matrix(10);
	let b = gen_b(10);
	println!("A₁₀ = \n{mat:5.2}");
	println!("b₁₀ = {b:.1}");
	let exact = mat.clone().lu_decompose().solve(&b);
	println!("x₁₀ = {exact:.6}");

	let mut x = Vector::new(10);
	println!("Jacobi: x₁₀⁽⁰⁾ = {x:.6}");

	for i in 1.. {
		x = mat.jacobi_iteration(&x, &b);
		let error = (&x - &exact).norm();
		println!("x₁₀⁽{}⁾ = {x:.6}, error = {error}", superscript(i));

		if error < f32::EPSILON.into() {
			break;
		}
	}

	let mut x = Vector::new(10);
	println!("Gauss-Seidel: x₁₀⁽⁰⁾ = {x:.6}");

	for i in 1.. {
		x = mat.gauss_seidel_iteration(&x, &b);
		let error = (&x - &exact).norm();
		println!("x₁₀⁽{}⁾ = {x:.6}, error = {error}", superscript(i));

		if error < f32::EPSILON.into() {
			break;
		}
	}

	let mat = gen_matrix(124);
	let b = gen_b(124);
	let exact = mat.clone().lu_decompose().solve(&b);

	let xs = iter::repeat(())
		.take(
			pico_args::Arguments::from_env()
				.opt_value_from_str("--starting-points")
				.unwrap()
				.unwrap_or(5),
		)
		.map(|_| Vector::from_iter(iter::repeat(()).take(124).map(|_| rand::random())))
		.collect::<Vec<_>>();

	plot_jacobi(&mat, &b, &exact, &xs);
	plot_gauss_seidel(&mat, &b, &exact, &xs);
}

fn plot_jacobi(
	mat: &BandMatrix<f64, 2, 2>,
	b: &Vector<f64>,
	exact: &Vector<f64>,
	xs: &[Vector<f64>],
) {
	let root = SVGBackend::new("./jacobi.svg", (1000, 500)).into_drawing_area();
	root.fill(&WHITE).unwrap();

	let mut chart = ChartBuilder::on(&root)
		.set_label_area_size(LabelAreaPosition::Left, 60)
		.set_label_area_size(LabelAreaPosition::Bottom, 60)
		.caption("Jacobi", ("sans-serif", 40))
		.build_cartesian_2d(0..81, (f32::EPSILON.into()..200.0).log_scale())
		.unwrap();

	chart.configure_mesh().draw().unwrap();

	for x in xs {
		let mut x = x.clone();

		let data: Vec<_> = {
			iter::once((&x - exact).norm())
				.chain(iter::from_fn(|| {
					x = mat.jacobi_iteration(&x, b);
					let error = (&x - exact).norm();

					if error < f32::EPSILON.into() {
						None
					} else {
						Some(error)
					}
				}))
				.collect()
		};

		chart
			.draw_series(LineSeries::new(
				(0..).zip(data.iter()).map(|(x, y)| (x, *y)),
				HSLColor(rand::random(), 0.9, 0.75),
			))
			.unwrap();
	}

	root.present().unwrap();
}

fn plot_gauss_seidel(
	mat: &BandMatrix<f64, 2, 2>,
	b: &Vector<f64>,
	exact: &Vector<f64>,
	xs: &[Vector<f64>],
) {
	let root = SVGBackend::new("./gauss-seidel.svg", (1000, 500)).into_drawing_area();
	root.fill(&WHITE).unwrap();

	let mut chart = ChartBuilder::on(&root)
		.set_label_area_size(LabelAreaPosition::Left, 60)
		.set_label_area_size(LabelAreaPosition::Bottom, 60)
		.caption("Gauss-Seidel", ("sans-serif", 40))
		.build_cartesian_2d(0..81, (f32::EPSILON.into()..200.0).log_scale())
		.unwrap();

	chart.configure_mesh().draw().unwrap();

	for x in xs {
		let mut x = x.clone();

		let data: Vec<_> = {
			iter::once((&x - exact).norm())
				.chain(iter::from_fn(|| {
					x = mat.gauss_seidel_iteration(&x, b);
					let error = (&x - exact).norm();

					if error < f32::EPSILON.into() {
						None
					} else {
						Some(error)
					}
				}))
				.collect()
		};

		chart
			.draw_series(LineSeries::new(
				(0..).zip(data.iter()).map(|(x, y)| (x, *y)),
				HSLColor(rand::random(), 0.9, 0.75),
			))
			.unwrap();
	}

	root.present().unwrap();
}
