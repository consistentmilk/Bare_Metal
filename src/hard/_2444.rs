///
/// 2444: Count Subarrays With Fixed Bounds
///
/// Intuition:
/// Scan the array once, tracking last seen positions of
/// minK, maxK, and the most recent invalid element. At
/// each index `i`, any subarray ending at `i` is valid if
/// it includes both bounds and no invalid values. The
/// count of such subarrays is
///   max(0, min(last_min, last_max) - last_invalid).
///
/// Algorithm:
/// 1. Initialize `last_min`, `last_max`, `last_invalid` to -1.
/// 2. Initialize `count` to 0.
/// 3. For each index `i` and value `x` in `nums`:
///    a. If `x < minK || x > maxK`, update `last_invalid = i`.
///    b. If `x == minK`, update `last_min = i`.
///    c. If `x == maxK`, update `last_max = i`.
///    d. Let `valid_start = min(last_min, last_max)`.
///    e. If `valid_start > last_invalid`, add
///       `(valid_start - last_invalid)` to `count`.
/// 4. Return `count`.
///
/// Time Complexity:
/// O(n), one pass through the array.
///
/// Space Complexity:
/// O(1), only constant extra variables.
pub struct Solution;

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, min_k: i32, max_k: i32) -> i64 {
        // Last index where we saw min_k
        let mut last_min: i32 = -1;

        // Last index where we saw max_k
        let mut last_max: i32 = -1;

        // Last index of any invalid element
        let mut last_invalid: i32 = -1;

        // Accumulated count of valid subarrays
        let mut count: i64 = 0i64;

        // Iterate through nums with index i
        for (i, &x) in nums.iter().enumerate() {
            // If x is outside [min_k, max_k], mark invalid
            if x < min_k || x > max_k {
                last_invalid = i as i32;
            }

            // Update last_min when we see min_k
            if x == min_k {
                last_min = i as i32;
            }

            // Update last_max when we see max_k
            if x == max_k {
                last_max = i as i32;
            }

            // Earliest index at which both bounds are included
            let valid_start: i32 = last_min.min(last_max);

            // Add the number of valid subarrays ending here
            if valid_start > last_invalid {
                count += (valid_start - last_invalid) as i64;
            }
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2444_1_two_equal_elements() {
        // nums = [1,1], minK == maxK == 1 →
        // subarrays: [1], [1], [1,1] → total 3
        let nums = vec![1, 1];
        let min_k = 1;
        let max_k = 1;
        let expected = 3; // corrected from 1 to 3
        assert_eq!(Solution::count_subarrays(nums, min_k, max_k), expected);
    }

    #[test]
    fn test_2444_2_two_elements_invalid() {
        // nums = [1,2], maxK = 5 never appears → none valid
        let nums = vec![1, 2];
        let min_k = 1;
        let max_k = 5;
        let expected = 0;
        assert_eq!(Solution::count_subarrays(nums, min_k, max_k), expected);
    }

    #[test]
    fn test_2444_3_simple_valid_pair() {
        // nums = [1,5], one minK and one maxK → one subarray
        let nums = vec![1, 5];
        let min_k = 1;
        let max_k = 5;
        let expected = 1;
        assert_eq!(Solution::count_subarrays(nums, min_k, max_k), expected);
    }

    #[test]
    fn test_2444_4_missing_max() {
        // nums = [1,2,3], maxK = 5 missing → none valid
        let nums = vec![1, 2, 3];
        let min_k = 1;
        let max_k = 5;
        let expected = 0;
        assert_eq!(Solution::count_subarrays(nums, min_k, max_k), expected);
    }

    #[test]
    fn test_2444_5_missing_min() {
        // nums = [3,5,5], minK = 1 missing → none valid
        let nums = vec![3, 5, 5];
        let min_k = 1;
        let max_k = 5;
        let expected = 0;
        assert_eq!(Solution::count_subarrays(nums, min_k, max_k), expected);
    }

    #[test]
    fn test_2444_6_example_one() {
        // Example 1 from prompt → 2 fixed-bound subarrays
        let nums = vec![1, 3, 5, 2, 7, 5];
        let min_k = 1;
        let max_k = 5;
        let expected = 2;
        assert_eq!(Solution::count_subarrays(nums, min_k, max_k), expected);
    }

    #[test]
    fn test_2444_7_example_two() {
        // Example 2 from prompt ([1,1,1,1]) → 10 subarrays
        let nums = vec![1, 1, 1, 1];
        let min_k = 1;
        let max_k = 1;
        let expected = 10;
        assert_eq!(Solution::count_subarrays(nums, min_k, max_k), expected);
    }

    #[test]
    fn test_2444_8_overlapping_bounds() {
        // nums = [1,5,1,5], overlapping fixed-bound windows
        let nums = vec![1, 5, 1, 5];
        let min_k = 1;
        let max_k = 5;
        let expected = 6;
        assert_eq!(Solution::count_subarrays(nums, min_k, max_k), expected);
    }

    #[test]
    fn test_2444_9_all_invalid() {
        // nums = [6,7,8], all > maxK → 0 valid subarrays
        let nums = vec![6, 7, 8];
        let min_k = 1;
        let max_k = 5;
        let expected = 0;
        assert_eq!(Solution::count_subarrays(nums, min_k, max_k), expected);
    }

    #[test]
    fn test_2444_10_mixed_case() {
        // nums = [1,2,1,5,1], computed via one-pass → 7
        let nums = vec![1, 2, 1, 5, 1];
        let min_k = 1;
        let max_k = 5;
        let expected = 7; // corrected from 8 to 7
        assert_eq!(Solution::count_subarrays(nums, min_k, max_k), expected);
    }
}
