use criterion::{black_box, criterion_group, criterion_main, Criterion};
use leetcode::easy::_027::Solution;

pub fn benchmark(c: &mut Criterion) {
    c.bench_function("_027", |b| {
        b.iter(|| {
            todo!();
        });
    });
}

criterion_group!(group_easy_027, benchmark);
criterion_main!(group_easy_027);

