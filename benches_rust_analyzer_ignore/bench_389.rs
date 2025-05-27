use criterion::{black_box, criterion_group, criterion_main, Criterion};
use leetcode::easy::_389::Solution;

pub fn benchmark(c: &mut Criterion) {
    c.bench_function("_389", |b| {
        b.iter(|| {
            todo!();
        });
    });
}

criterion_group!(group_easy_389, benchmark);
criterion_main!(group_easy_389);

