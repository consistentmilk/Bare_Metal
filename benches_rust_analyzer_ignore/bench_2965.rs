use criterion::{black_box, criterion_group, criterion_main, Criterion};
use leetcode::easy::_2965::Solution;

pub fn benchmark(c: &mut Criterion) {
    c.bench_function("_2965", |b| {
        b.iter(|| {
            todo!();
        });
    });
}

criterion_group!(group_easy_2965, benchmark);
criterion_main!(group_easy_2965);

