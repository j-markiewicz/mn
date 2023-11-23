use criterion::{
	criterion_group, criterion_main, AxisScale, BatchSize, BenchmarkId, Criterion,
	PlotConfiguration, Throughput,
};
use num3::gen_matrix;

fn bench(c: &mut Criterion) {
	let mut group = c.benchmark_group("LU decomposition");
	group.plot_config(PlotConfiguration::default().summary_scale(AxisScale::Logarithmic));

	for exp in 1..=18 {
		let size = 2usize.pow(exp);
		group.throughput(Throughput::Elements(size as u64));

		group.bench_with_input(
			BenchmarkId::new("with band optimization", size),
			&size,
			|b, &size| {
				b.iter_batched(
					|| gen_matrix(size),
					|matrix| matrix.lu_decompose(),
					BatchSize::SmallInput,
				);
			},
		);

		if size < 20000 {
			group.bench_with_input(
				BenchmarkId::new("without band optimization", size),
				&size,
				|b, &size| {
					b.iter_batched(
						|| gen_matrix(size),
						|matrix| matrix.lu_decompose_slow(),
						BatchSize::SmallInput,
					);
				},
			);
		}
	}
	group.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);
