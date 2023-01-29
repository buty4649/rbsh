extern crate rbsh_parser;
use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let str = r#"
if true
  echo OK
else
  echo NG
end
"#;

    c.bench_function("parser", |b| b.iter(|| rbsh_parser::parse(str, true)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
