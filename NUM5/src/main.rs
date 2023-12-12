use std::iter;

use plotters::{coord::types::RangedCoordi32, prelude::*};

use num5::{gen_b, gen_matrix, BandMatrix, Vector};

type Chart<'a, 'b> = ChartContext<'a, SVGBackend<'b>, Cartesian2d<RangedCoordi32, LogCoord<f64>>>;

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

	println!("x₁₂₄ = {exact:.6}");

	let xs = iter::repeat(())
		.take(
			pico_args::Arguments::from_env()
				.opt_value_from_str("--starting-points")
				.unwrap()
				.unwrap_or(5),
		)
		.map(|_| Vector::from_iter(iter::repeat(()).take(124).map(|_| rand::random())))
		.collect::<Vec<_>>();

	let root = SVGBackend::new("./errors.svg", (1000, 500)).into_drawing_area();
	root.fill(&WHITE).unwrap();

	let mut chart = ChartBuilder::on(&root)
		.set_label_area_size(LabelAreaPosition::Left, 60)
		.set_label_area_size(LabelAreaPosition::Bottom, 60)
		.build_cartesian_2d(0..81, (f32::EPSILON.into()..200.0).log_scale())
		.unwrap();

	chart
		.configure_mesh()
		.x_desc("k")
		.y_desc("||x⁽ᵏ⁾ - x||")
		.draw()
		.unwrap();

	plot_jacobi(&mut chart, &mat, &b, &exact, &xs);
	plot_gauss_seidel(&mut chart, &mat, &b, &exact, &xs);

	chart
		.configure_series_labels()
		.border_style(&BLACK)
		.background_style(&WHITE.mix(0.8))
		.draw()
		.unwrap();

	root.present().unwrap();
}

fn plot_jacobi(
	chart: &mut Chart<'_, '_>,
	mat: &BandMatrix<f64, 2, 2>,
	b: &Vector<f64>,
	exact: &Vector<f64>,
	xs: &[Vector<f64>],
) {
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

		let color = HSLColor((rand::random::<f64>() + 1.0) / 4.0, 0.9, 0.75);
		chart
			.draw_series(LineSeries::new(
				(0..).zip(data.iter()).map(|(x, y)| (x, *y)),
				color,
			))
			.unwrap()
			.label("Jacobi")
			.legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &color));
	}
}

fn plot_gauss_seidel(
	chart: &mut Chart<'_, '_>,
	mat: &BandMatrix<f64, 2, 2>,
	b: &Vector<f64>,
	exact: &Vector<f64>,
	xs: &[Vector<f64>],
) {
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

		let color = HSLColor((rand::random::<f64>() + 3.0) / 4.0, 0.9, 0.75);
		chart
			.draw_series(LineSeries::new(
				(0..).zip(data.iter()).map(|(x, y)| (x, *y)),
				color,
			))
			.unwrap()
			.label("Gauss-Seidel")
			.legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &color));
	}
}
