use criterion::{criterion_group, criterion_main, Criterion};
use speak::{run, learn, Map};

fn criterion_benchmark(c: &mut Criterion) {
	let mut map: Map<String> = Map::new();
	for i in 0..800 {
		map.push(i.to_string(), i.to_string())
	}
	c.bench_function("Speak Learn", |b| b.iter(|| learn(&map, None)));
	c.bench_function("Speak Run", |b| b.iter(|| run("Hi", &learn(&map, None), None, None, None, None)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);