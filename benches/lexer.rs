use std::fs::read_to_string;

use calcagebra_lib::lexer::Lexer;
use criterion::{Criterion, criterion_group, criterion_main};

fn criterion_benchmark(c: &mut Criterion) {
	let contents = read_to_string("benches/lexer.txt").unwrap();

	c.bench_function("lexer", |b| b.iter(|| Lexer::new(&contents).tokens()));
}

criterion_group!(name = benches; config = Criterion::default().sample_size(1000); targets = criterion_benchmark);
criterion_main!(benches);
