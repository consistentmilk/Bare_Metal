use criterion::{black_box, criterion_group, criterion_main, Criterion};
use leetcode::easy::_028::Solution;

pub fn benchmark(c: &mut Criterion) {
    c.bench_function("_028", |b| {
        b.iter(|| {
            todo!();
        });
    });
}

criterion_group!(group_easy_028, benchmark);
criterion_main!(group_easy_028);

