use std::fs::read_to_string;

use calcagebra_lib::{interpreter::Interpreter, lexer::Lexer, parser::Parser};
use criterion::{Criterion, criterion_group, criterion_main};

fn criterion_benchmark(c: &mut Criterion) {
	let lex = read_to_string("benches/lexer.cal").unwrap();
	let parse = Lexer::new(&read_to_string("benches/parser.cal").unwrap()).tokens();

	c.bench_function("lexer", |b| b.iter(|| Lexer::new(&lex).tokens()));

	c.bench_function("parser", |b| b.iter(|| Parser::new(&parse).ast()));

	c.bench_function("interpreter", |b| {
		b.iter_batched(
			|| {
				Parser::new(&Lexer::new(&read_to_string("benches/interpreter.cal").unwrap()).tokens())
					.ast()
					.unwrap()
			},
			|ast| Interpreter::new().interpret(ast),
			criterion::BatchSize::SmallInput,
		)
	});
}

criterion_group!(name = benches; config = Criterion::default().sample_size(1000); targets = criterion_benchmark);
criterion_main!(benches);
