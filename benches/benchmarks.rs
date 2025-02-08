use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::Rng;

// Define the two implementations
pub fn is_array_special_xor(nums: Vec<i32>) -> bool {
    let n: usize = nums.len();
    let mut ptr: usize = 1;

    loop {
        if ptr == n {
            break;
        }

        if (nums[ptr - 1] ^ nums[ptr]) & 1 != 1 {
            return false;
        }

        ptr += 1;
    }

    true
}

pub fn is_array_special_add(nums: Vec<i32>) -> bool {
    let n: usize = nums.len();
    let mut ptr: usize = 1;

    loop {
        if ptr == n {
            break;
        }

        if (nums[ptr - 1] + nums[ptr]) % 2 == 0 {
            return false;
        }

        ptr += 1;
    }

    true
}

// Generate a random array of even and odd numbers
fn generate_random_array(size: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..size).map(|_| rng.gen_range(0..100)).collect()
}

// Benchmark function
fn benchmark_functions(c: &mut Criterion) {
    let input = generate_random_array(1000); // Adjust size as needed

    c.bench_function("is_array_special_xor", |b| {
        b.iter(|| is_array_special_xor(black_box(input.clone())))
    });

    c.bench_function("is_array_special_add", |b| {
        b.iter(|| is_array_special_add(black_box(input.clone())))
    });
}

criterion_group!(benches, benchmark_functions);
criterion_main!(benches);

// use leetcode::hard::_004;
// use leetcode::medium::_003;

// fn main() {
//     divan::main();
// }

// #[divan::bench]
// fn bench_004_optimized_binary_search() {
//     let nums1 = vec![1, 3, 5, 7, 9];
//     let nums2 = vec![2, 4, 6, 8, 10];
//     _004::Solution::find_median_sorted_arrays(nums1, nums2);
// }

// #[divan::bench]
// fn bench_004_merge_sort() {
//     let nums1 = vec![1, 3, 5, 7, 9];
//     let nums2 = vec![2, 4, 6, 8, 10];
//     _004::Solution::find_median_sorted_arrays_alt_2(nums1, nums2);
// }

// #[divan::bench]
// fn bench_003_hashmap() {
//     let test: String = "abcabcbb".to_string();

//     _003::Solution::length_of_longest_substring(test);
// }

// #[divan::bench]
// fn bench_003_fixed_array() {
//     let test: String = "abcabcbb".to_string();

//     _003::Solution::length_of_longest_substring_opt(test);
// }
