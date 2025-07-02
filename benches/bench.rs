use std::fs::read_to_string;

use calcagebra_lib::{lexer::Lexer, parser::Parser};
use criterion::{Criterion, criterion_group, criterion_main};

fn criterion_benchmark(c: &mut Criterion) {
	let lex = read_to_string("benches/lexer.txt").unwrap();
	let parse = Lexer::new(&read_to_string("benches/parser.txt").unwrap()).tokens();

	c.bench_function("lexer", |b| b.iter(|| Lexer::new(&lex).tokens()));

	c.bench_function("parser", |b| b.iter(|| Parser::new(&parse).ast()));
}

criterion_group!(name = benches; config = Criterion::default().sample_size(1000); targets = criterion_benchmark);
criterion_main!(benches);
