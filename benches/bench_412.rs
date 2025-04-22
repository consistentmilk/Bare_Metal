use criterion::{Criterion, black_box, criterion_group, criterion_main};
use leetcode::easy::_412::Solution;
use std::time::Duration;

/// Benchmark for Problem 412: Fizz Buzz
/// Group: "fizz_buzz_easy_412"
/// Functions: "fizz_buzz" and "fizz_buzz_unsafe_optimized"
pub fn benchmark(c: &mut Criterion) {
    // Create and configure the benchmark group
    let mut group = c.benchmark_group("fizz_buzz_easy_412");
    group.measurement_time(Duration::from_secs(3));

    // Use a large n to simulate realistic performance
    let test_n: i32 = 100_000;

    // Benchmark the standard fizz_buzz implementation
    group.bench_function("fizz_buzz", |b| {
        b.iter(|| {
            let _ = Solution::fizz_buzz(black_box(test_n));
        });
    });

    // Benchmark the unsafe optimized fizz_buzz implementation
    group.bench_function("fizz_buzz_unsafe_optimized", |b| {
        b.iter(|| {
            let _ = Solution::fizz_buzz_unsafe_optimized(black_box(test_n));
        });
    });

    group.finish();
}

criterion_group!(group_easy_412, benchmark);
criterion_main!(group_easy_412);
