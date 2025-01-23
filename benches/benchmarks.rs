use leetcode::hard::_004;
use leetcode::medium::_003;

fn main() {
    divan::main();
}

#[divan::bench]
fn bench_004_optimized_binary_search() {
    let nums1 = vec![1, 3, 5, 7, 9];
    let nums2 = vec![2, 4, 6, 8, 10];
    _004::Solution::find_median_sorted_arrays(nums1, nums2);
}

#[divan::bench]
fn bench_004_merge_sort() {
    let nums1 = vec![1, 3, 5, 7, 9];
    let nums2 = vec![2, 4, 6, 8, 10];
    _004::Solution::find_median_sorted_arrays_alt_2(nums1, nums2);
}

#[divan::bench]
fn bench_003_hashmap() {
    let test: String = "abcabcbb".to_string();

    _003::Solution::length_of_longest_substring(test);
}

#[divan::bench]
fn bench_003_fixed_array() {
    let test: String = "abcabcbb".to_string();

    _003::Solution::length_of_longest_substring_opt(test);
}
