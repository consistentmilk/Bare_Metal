use criterion::{Criterion, black_box, criterion_group, criterion_main};
use leetcode::easy::_1002::Solution;

pub fn benchmark(c: &mut Criterion) {
    c.bench_function("_1002", |b| {
        b.iter(|| {
            todo!();
        });
    });
}

criterion_group!(group_easy_1002, benchmark);
criterion_main!(group_easy_1002);
