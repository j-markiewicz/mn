use criterion::{
	criterion_group, criterion_main, AxisScale, BatchSize, BenchmarkId, Criterion,
	PlotConfiguration, Throughput,
};
use num4::{gen_b, gen_matrix, solve};

fn bench(c: &mut Criterion) {
	let mut group = c.benchmark_group("Solve");
	group.plot_config(PlotConfiguration::default().summary_scale(AxisScale::Logarithmic));

	for exp in 1..=16 {
		let size = 2usize.pow(exp);
		group.throughput(Throughput::Elements(size as u64));

		group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
			b.iter_batched(
				|| (gen_matrix(size), gen_b(size)),
				|(mat, b)| solve(mat, b),
				BatchSize::SmallInput,
			);
		});
	}
	group.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);
