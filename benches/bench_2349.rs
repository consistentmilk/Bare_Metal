use criterion::{Criterion, black_box, criterion_group, criterion_main};
use leetcode::medium::_2349::NumberContainers;

pub fn benchmark(c: &mut Criterion) {
    c.bench_function("_2349", |b| {
        b.iter(|| {
            todo!();
        });
    });
}

criterion_group!(group_medium_2349, benchmark);
criterion_main!(group_medium_2349);
