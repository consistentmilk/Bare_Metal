use criterion::{black_box, criterion_group, criterion_main, Criterion};
use leetcode::easy::_013::Solution;

pub fn benchmark(c: &mut Criterion) {
    c.bench_function("_013", |b| {
        b.iter(|| {
            todo!();
        });
    });
}

criterion_group!(group_easy_013, benchmark);
criterion_main!(group_easy_013);

