/*
Problem 704: Binary Search

Intuition
---------
- We have a sorted array of distinct integers.
- We need to determine if `target` exists.
- Binary search splits the searchable range in half each step.
- Compare `nums[mid]` with `target`:
  • If equal, return `mid`.
  • If `nums[mid] < target`, search right half.
  • Otherwise, search left half.
- This yields O(log n) time.

Algorithm
---------
1. Initialize `left = 0`, `right = nums.len() - 1`.
2. While `left <= right`:
   a. Compute `mid = left + (right - left) / 2`.
   b. If `nums[mid] == target`, return `mid`.
   c. If `nums[mid] < target`, `left = mid + 1`.
   d. Else, `right = mid - 1`.
3. If loop ends, `target` not found; return -1.

Time Complexity
---------------
O(log n), where n = nums.len().

Space Complexity
----------------
O(1), constant extra space.
*/

pub struct Solution;

impl Solution {
    /// Searches for `target` in `nums` using binary search.
    ///
    /// # Arguments
    /// * `nums`   – sorted vector of distinct integers
    /// * `target` – integer to find
    ///
    /// # Returns
    /// * index of `target`, or -1 if not found
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        // initialize left pointer
        let mut left: i32 = 0;

        // initialize right pointer
        let mut right: i32 = nums.len() as i32 - 1;

        // binary search loop
        while left <= right {
            // compute mid without overflow
            let mid: i32 = left + (right - left) / 2;

            // found target?
            if nums[mid as usize] == target {
                return mid;
            }

            // decide which half to continue searching
            if nums[mid as usize] < target {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }

        // target not found
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_704_1_found_middle() {
        let nums = vec![-1, 0, 3, 5, 9, 12];
        let target = 9;
        let expected = 4;
        assert_eq!(Solution::search(nums, target), expected);
    }

    #[test]
    fn test_704_2_not_found() {
        let nums = vec![-1, 0, 3, 5, 9, 12];
        let target = 2;
        let expected = -1;
        assert_eq!(Solution::search(nums, target), expected);
    }

    #[test]
    fn test_704_3_found_first() {
        let nums = vec![1, 2, 3, 4, 5];
        let target = 1;
        let expected = 0;
        assert_eq!(Solution::search(nums, target), expected);
    }

    #[test]
    fn test_704_4_found_last() {
        let nums = vec![1, 2, 3, 4, 5];
        let target = 5;
        let expected = 4;
        assert_eq!(Solution::search(nums, target), expected);
    }
}
