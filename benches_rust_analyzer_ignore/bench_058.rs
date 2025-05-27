use criterion::{black_box, criterion_group, criterion_main, Criterion};
use leetcode::easy::_058::Solution;

pub fn benchmark(c: &mut Criterion) {
    c.bench_function("_058", |b| {
        b.iter(|| {
            todo!();
        });
    });
}

criterion_group!(group_easy_058, benchmark);
criterion_main!(group_easy_058);

