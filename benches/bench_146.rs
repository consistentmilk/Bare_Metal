use criterion::{Criterion, black_box, criterion_group, criterion_main};
use leetcode::medium::_146::LRUCache;

pub fn benchmark(c: &mut Criterion) {
    c.bench_function("_146", |b| {
        b.iter(|| {
            todo!();
        });
    });
}

criterion_group!(group_medium_146, benchmark);
criterion_main!(group_medium_146);
