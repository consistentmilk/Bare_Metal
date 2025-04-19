use criterion::{black_box, criterion_group, criterion_main, Criterion};
use leetcode::easy::_1137::Solution;

pub fn benchmark(c: &mut Criterion) {
    c.bench_function("_1137", |b| {
        b.iter(|| {
            todo!();
        });
    });
}

criterion_group!(group_easy_1137, benchmark);
criterion_main!(group_easy_1137);

