use criterion::{black_box, criterion_group, criterion_main, Criterion};
use leetcode::easy::_168::Solution;

pub fn benchmark(c: &mut Criterion) {
    c.bench_function("_168", |b| {
        b.iter(|| {
            todo!();
        });
    });
}

criterion_group!(group_easy_168, benchmark);
criterion_main!(group_easy_168);

