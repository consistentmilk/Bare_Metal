use criterion::{black_box, criterion_group, criterion_main, Criterion};
use leetcode::easy::_3151::Solution;

pub fn benchmark(c: &mut Criterion) {
    c.bench_function("_3151", |b| {
        b.iter(|| {
            todo!();
        });
    });
}

criterion_group!(group_easy_3151, benchmark);
criterion_main!(group_easy_3151);

