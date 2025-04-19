use criterion::{Criterion, black_box, criterion_group, criterion_main};
use leetcode::hard::_3165::Solution;

pub fn benchmark(c: &mut Criterion) {
    c.bench_function("_3165", |b| {
        b.iter(|| {
            todo!();
        });
    });
}

criterion_group!(group_hard_3165, benchmark);
criterion_main!(group_hard_3165);
