use criterion::{Criterion, black_box, criterion_group, criterion_main};
use leetcode::medium::_002::Solution;
use std::time::Duration;

use leetcode::utils::list::*;

/// Benchmark for Problem 002: Add Two Numbers (Linked List Sum)
/// Group: "add_two_numbers_medium_002"
/// Function: "add_two_numbers"
pub fn benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("add_two_numbers_medium_002");
    group.measurement_time(Duration::from_secs(3));

    // Prepare inputs: long lists to simulate large addition
    let l1 = ListNode::list_maker(vec![9; 1000]);
    let l2 = ListNode::list_maker(vec![1]);

    group.bench_function("add_two_numbers", |b| {
        b.iter(|| {
            // We clone because the function consumes the input
            let l1c = black_box(l1.clone());
            let l2c = black_box(l2.clone());

            let _ = Solution::add_two_numbers(l1c, l2c);
        });
    });

    group.finish();
}

criterion_group!(group_medium_002, benchmark);
criterion_main!(group_medium_002);
