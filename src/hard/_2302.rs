/*
Intuition
---------
Use a sliding window.  Maintain the smallest left index such that the
current window [left, right] has a score strictly below `k`.  Every
shorter subarray ending at `right` is automatically valid, allowing us
to add their count in O(1).

Algorithm
---------
See the step-by-step comment above.  The core loop performs:
  • Expand window by including nums[right].
  • While the score is ≥ k, contract window from the left.
  • Add the window length to the running answer.

Time Complexity
---------------
O(n) — each array element is visited at most twice, once when right
extends the window and once when left removes it.

Space Complexity
----------------
O(1) — constant auxiliary storage.

*/

pub struct Solution;

impl Solution {
    /// Counts sub-arrays whose (sum · length) is strictly smaller than `k`.
    ///
    /// # Arguments
    /// * `nums` – non-empty vector of positive integers
    /// * `k`    – positive upper bound on the sub-array score
    ///
    /// # Returns
    /// * `i64`  – number of qualifying sub-arrays (may exceed `u32`)
    pub fn count_subarrays(nums: Vec<i32>, k: i64) -> i64 {
        // Running sum of elements inside current window [left, right]
        let mut window_sum: i64 = 0;

        // Left boundary of current window
        let mut left: usize = 0;

        // Final answer (can be up to n*(n+1)/2, hence i64)
        let mut count: i64 = 0;

        // Iterate over every possible right boundary
        for (right, &value) in nums.iter().enumerate() {
            // Extend window to include nums[right]
            window_sum += value as i64;

            // Shrink window until its score falls below k
            while window_sum * (right as i64 - left as i64 + 1) >= k {
                // remove nums[left]
                window_sum -= nums[left] as i64;

                // advance left border
                left += 1;
            }

            // All windows starting at indices >= left and ending at right
            // are now valid; their count is window length
            count += (right - left + 1) as i64;
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    // -------------------- helpers --------------------
    fn run(nums: &[i32], k: i64) -> i64 {
        Solution::count_subarrays(nums.to_vec(), k)
    }

    // -------------------- unit tests -----------------
    #[test]
    fn test_2302_1_single_elements() {
        assert_eq!(run(&[2, 1, 4, 3, 5], 10), 6);
    }

    #[test]
    fn test_2302_2_all_equal_small_k() {
        assert_eq!(run(&[1, 1, 1], 5), 5);
    }

    #[test]
    fn test_2302_3_entire_array_invalid() {
        assert_eq!(run(&[5, 5, 5], 10), 3);
    }

    #[test]
    fn test_2302_4_k_very_large() {
        let nums = vec![100_000; 50];
        // All subarrays fit when k is huge
        let n = nums.len() as i64;
        assert_eq!(run(&nums, 10_i64.pow(15)), n * (n + 1) / 2);
    }
}
