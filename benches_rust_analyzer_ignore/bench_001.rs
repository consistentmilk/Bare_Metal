use criterion::{Criterion, black_box, criterion_group, criterion_main};
use leetcode::easy::_001::Solution;
use std::time::Duration;

/// Benchmark for Problem 001: Two Sum
/// Group name: "two_sum_easy_001"
/// Function name: "two_sum"
pub fn benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("two_sum_easy_001");
    group.measurement_time(Duration::from_secs(3));

    group.bench_function("two_sum", |b| {
        b.iter(|| {
            let nums = black_box(vec![3, 2, 4]);
            let target = black_box(6);
            let _ = Solution::two_sum(nums, target);
        });
    });

    group.finish();
}

criterion_group!(group_easy_001, benchmark);
criterion_main!(group_easy_001);
