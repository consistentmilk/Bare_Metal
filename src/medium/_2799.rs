/*!
Intuition:
  Each complete subarray must contain all K distinct values
  present in the entire array. As soon as a window [left..right)
  first reaches K distinct values, any extension of that window
  will also be complete.

Algorithm:
  1. Compute K = total distinct values in `nums`.
  2. Prepare a frequency buffer `freq` of size 2001 (max num+1).
  3. Use two pointers, `left` and `right`, and a running count
     `distinct_win` for the window’s distinct elements.
  4. For each `left` from 0 to n-1:
     a. Advance `right` until `distinct_win == K` or `right == n`.
     b. If `distinct_win < K`, break out (no further complete
        windows possible).
     c. Add `(n - right + 1)` to `ans` (all extensions are valid).
     d. Remove `nums[left]` from window, adjust `freq` and
        `distinct_win`.
  5. Return `ans`.

Time Complexity: O(n + U) ≈ O(n), where U = 2001.
Space Complexity: O(U), for the `freq` buffer.
*/
use std::collections::HashSet;

pub struct Solution;

impl Solution {
    /// Count subarrays that contain all distinct values of `nums`
    pub fn count_complete_subarrays(nums: Vec<i32>) -> i32 {
        // n = length of input array
        let n: usize = nums.len();

        // Compute total distinct values K
        let mut set: HashSet<i32> = HashSet::new();

        for &v in &nums {
            set.insert(v);
        }

        let k: usize = set.len();

        // Frequency buffer for values 0..2000
        let mut freq: Vec<usize> = vec![0; 2001];

        // distinct_win = current window's distinct count
        let mut distinct_win: usize = 0;

        // ans = total count of complete subarrays
        let mut ans: usize = 0;

        // right pointer (exclusive)
        let mut right: usize = 0;

        // Slide left pointer from 0 to n-1
        for left in 0..n {
            // Expand right until window has K distinct or end reached
            while right < n && distinct_win < k {
                // Current value at right
                let v: usize = nums[right] as usize;

                // If first time seeing v in window, increment distinct_win
                if freq[v] == 0 {
                    distinct_win += 1;
                }

                // Increment frequency of v
                freq[v] += 1;

                // Move right forward
                right += 1;
            }

            // If we never reached K distinct, no more complete windows
            if distinct_win < k {
                break;
            }

            // All windows [left..r] for r in right..=n are complete
            ans += n - right + 1;

            // Remove nums[left] from window before next iteration
            let lv: usize = nums[left] as usize;
            freq[lv] -= 1;

            // If frequency drops to zero, decrease distinct_win
            if freq[lv] == 0 {
                distinct_win -= 1;
            }
        }

        // Cast to i32 as result fits within range
        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2799_1_example1() {
        // Example 1: multiple complete subarrays
        let nums = vec![1, 3, 1, 2, 2];
        // Expect 4 complete subarrays
        assert_eq!(Solution::count_complete_subarrays(nums), 4);
    }

    #[test]
    fn test_2799_2_example2() {
        // Example 2: all elements identical
        let nums = vec![5, 5, 5, 5];
        // Expect 10 complete subarrays
        assert_eq!(Solution::count_complete_subarrays(nums), 10);
    }

    #[test]
    fn test_2799_3_single() {
        // Single element produces one complete subarray
        let nums = vec![7];
        // Expect 1 complete subarray
        assert_eq!(Solution::count_complete_subarrays(nums), 1);
    }

    #[test]
    fn test_2799_4_all_distinct() {
        // All distinct: only the full array is complete
        let nums = vec![1, 2, 3];
        // Expect 1 complete subarray
        assert_eq!(Solution::count_complete_subarrays(nums), 1);
    }

    #[test]
    fn test_2799_5_mixed() {
        // Mixed pattern with three distinct values
        let nums = vec![1, 2, 1, 2, 3];
        // Expect 3 complete subarrays
        assert_eq!(Solution::count_complete_subarrays(nums), 3);
    }
}
