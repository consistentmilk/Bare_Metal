use criterion::{black_box, criterion_group, criterion_main, Criterion};
use leetcode::medium::_017::Solution;

pub fn benchmark(c: &mut Criterion) {
    c.bench_function("_017", |b| {
        b.iter(|| {
            todo!();
        });
    });
}

criterion_group!(group_medium_017, benchmark);
criterion_main!(group_medium_017);

