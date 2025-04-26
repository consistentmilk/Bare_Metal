use criterion::{Criterion, black_box, criterion_group, criterion_main};
use leetcode::medium::_002::Solution;
use leetcode::utils::list::*;
use std::time::Duration;

pub fn benchmark(c: &mut Criterion) {
    // Create a group for Problem 002: Add Two Numbers
    let mut group = c.benchmark_group("add_two_numbers_medium_002");
    // Run each measurement for 3 seconds to reduce noise
    group.measurement_time(Duration::from_secs(3));

    // Prepare inputs: list of 1000 nines and a single one
    let l1 = ListNode::list_maker(vec![9; 1000]);
    let l2 = ListNode::list_maker(vec![1]);

    // Benchmark the safe implementation
    group.bench_function("add_two_numbers_safe", |b| {
        b.iter(|| {
            let l1c = black_box(l1.clone());
            let l2c = black_box(l2.clone());
            let _ = Solution::add_two_numbers(l1c, l2c);
        });
    });

    // Benchmark the unsafe implementation
    group.bench_function("add_two_numbers_unsafe", |b| {
        b.iter(|| {
            let l1c = black_box(l1.clone());
            let l2c = black_box(l2.clone());
            let _ = Solution::add_two_numbers_unsafe(l1c, l2c);
        });
    });

    group.finish();
}

criterion_group!(group_medium_002, benchmark);
criterion_main!(group_medium_002);
