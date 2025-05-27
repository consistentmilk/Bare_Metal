use criterion::{Criterion, black_box, criterion_group, criterion_main};
use leetcode::medium::_003::Solution;
use std::time::Duration;

/// Benchmark for Problem 003: Longest Substring Without Repeating Characters
/// Group: "longest_substring_medium_003"
/// Function: "length_of_longest_substring"
pub fn benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("longest_substring_medium_003");
    group.measurement_time(Duration::from_secs(3));

    // Create a long test string to simulate realistic performance
    let test_string: String = "abcdefghijklmnopqrstuvwxyz".repeat(100); // 2600 chars, no repeats

    group.bench_function("length_of_longest_substring", |b| {
        b.iter(|| {
            let input = black_box(test_string.clone());
            let _ = Solution::length_of_longest_substring(input);
        });
    });

    group.bench_function("length_of_longest_substring_opt", |b| {
        b.iter(|| {
            let input = black_box(test_string.clone());
            let _ = Solution::length_of_longest_substring_opt(input);
        });
    });

    group.finish();
}

criterion_group!(group_medium_003, benchmark);
criterion_main!(group_medium_003);
