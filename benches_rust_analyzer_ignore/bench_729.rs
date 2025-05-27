use criterion::{black_box, criterion_group, criterion_main, Criterion};
use leetcode::medium::_729::MyCalendar;

pub fn benchmark(c: &mut Criterion) {
    c.bench_function("_729", |b| {
        b.iter(|| {
            todo!();
        });
    });
}

criterion_group!(group_medium_729, benchmark);
criterion_main!(group_medium_729);

