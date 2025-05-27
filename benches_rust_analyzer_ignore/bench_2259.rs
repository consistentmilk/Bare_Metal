use criterion::{black_box, criterion_group, criterion_main, Criterion};
use leetcode::easy::_2259::Solution;

pub fn benchmark(c: &mut Criterion) {
    c.bench_function("_2259", |b| {
        b.iter(|| {
            todo!();
        });
    });
}

criterion_group!(group_easy_2259, benchmark);
criterion_main!(group_easy_2259);

