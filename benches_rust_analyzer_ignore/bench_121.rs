use criterion::{black_box, criterion_group, criterion_main, Criterion};
use leetcode::easy::_121::Solution;

pub fn benchmark(c: &mut Criterion) {
    c.bench_function("_121", |b| {
        b.iter(|| {
            todo!();
        });
    });
}

criterion_group!(group_easy_121, benchmark);
criterion_main!(group_easy_121);

