/***********************************************************************
Intuition
---------
We are tasked with finding the median of two sorted arrays, `nums1`
and `nums2`, without merging them fully. Merging would take O(m+n)
time and space, which is too slow.

Instead, the key idea is:
- Find a partition between the two arrays such that:
  - All elements to the left of the partition are less than or equal
    to all elements to the right.
- The left and right parts together divide the merged arrays into
  halves.
- The median is then simply:
  - The maximum of the left parts if the total number of elements
    is odd.
  - The average of the maximum of the left parts and the minimum
    of the right parts if even.

We can use **binary search** over the smaller array to efficiently
find the correct partition, achieving O(log(min(m, n))) time.

Algorithm (Step-by-Step)
------------------------
1. Let m = nums1.len(), n = nums2.len().
2. Ensure nums1 is the smaller array. If not, swap nums1 and nums2.
3. Initialize binary search boundaries:
   - left = 0
   - right = m
4. Perform binary search:
   - partition_a = (left + right) / 2
   - partition_b = (m + n + 1) / 2 - partition_a
5. Find:
   - max_left_a = nums1[partition_a - 1] (or -∞ if partition_a == 0)
   - min_right_a = nums1[partition_a] (or +∞ if partition_a == m)
   - max_left_b = nums2[partition_b - 1] (or -∞ if partition_b == 0)
   - min_right_b = nums2[partition_b] (or +∞ if partition_b == n)
6. Check if partition is valid:
   - max_left_a <= min_right_b
   - max_left_b <= min_right_a
7. If partition is valid:
   - If total elements odd → median = max(max_left_a, max_left_b)
   - Else → median = (max(max_left_a, max_left_b) + min(min_right_a,
     min_right_b)) / 2
8. Else:
   - If max_left_a > min_right_b → move left (right = partition_a - 1)
   - Else → move right (left = partition_a + 1)

Time Complexity
---------------
- O(log(min(m, n)))

Space Complexity
----------------
- O(1)
***********************************************************************/
pub struct Solution;

impl Solution {
    /// Returns the median of two sorted arrays without full merging.
    ///
    /// # Arguments
    /// * `nums1` – A sorted vector of integers
    /// * `nums2` – Another sorted vector of integers
    ///
    /// # Returns
    /// * A floating-point number representing the median
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        // Capture lengths of nums1 and nums2
        let m: usize = nums1.len();
        let n: usize = nums2.len();

        // Always binary search on the smaller array
        if m > n {
            return Self::find_median_sorted_arrays(nums2, nums1);
        }

        // Initialize binary search bounds on nums1
        let mut left: usize = 0;
        let mut right: usize = m;

        // Perform binary search loop
        while left <= right {
            // Partition nums1 at partition_a
            let partition_a: usize = (left + right) / 2;
            // Partition nums2 so that left side has (m+n+1)/2 elements
            let partition_b: usize = (m + n + 1) / 2 - partition_a;

            // Find the maximum value on the left side of partition_a
            let max_left_a: i32 = if partition_a == 0 {
                i32::MIN
            } else {
                nums1[partition_a - 1]
            };

            // Find the minimum value on the right side of partition_a
            let min_right_a: i32 = if partition_a == m {
                i32::MAX
            } else {
                nums1[partition_a]
            };

            // Find the maximum value on the left side of partition_b
            let max_left_b: i32 = if partition_b == 0 {
                i32::MIN
            } else {
                nums2[partition_b - 1]
            };

            // Find the minimum value on the right side of partition_b
            let min_right_b: i32 = if partition_b == n {
                i32::MAX
            } else {
                nums2[partition_b]
            };

            // Check if the current partition is valid
            if (max_left_a <= min_right_b) && (max_left_b <= min_right_a) {
                // If total elements are even
                if (m + n) % 2 == 0 {
                    // Return the average of max(left) and min(right)
                    return f64::from(max_left_a.max(max_left_b) + min_right_a.min(min_right_b))
                        / 2.0;
                } else {
                    // Return the maximum of the left side if odd
                    return f64::from(max_left_a.max(max_left_b));
                }
            } else if max_left_a > min_right_b {
                // Move binary search window to the left
                right = partition_a - 1;
            } else {
                // Move binary search window to the right
                left = partition_a + 1;
            }
        }

        // This line should never be reached if inputs are valid
        0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_004_1() {
        let nums1 = vec![1, 3];
        let nums2 = vec![2];
        let expected = 2.0;
        assert_eq!(Solution::find_median_sorted_arrays(nums1, nums2), expected);
    }

    #[test]
    fn test_004_2() {
        let nums1 = vec![1, 2];
        let nums2 = vec![3, 4];
        let expected = 2.5;
        assert_eq!(Solution::find_median_sorted_arrays(nums1, nums2), expected);
    }

    #[test]
    fn test_004_3() {
        let nums1 = vec![];
        let nums2 = vec![1];
        let expected = 1.0;
        assert_eq!(Solution::find_median_sorted_arrays(nums1, nums2), expected);
    }

    #[test]
    fn test_004_4() {
        let nums1 = vec![];
        let nums2 = vec![2, 3];
        let expected = 2.5;
        assert_eq!(Solution::find_median_sorted_arrays(nums1, nums2), expected);
    }

    #[test]
    fn test_004_5() {
        let nums1 = vec![1];
        let nums2 = vec![];
        let expected = 1.0;
        assert_eq!(Solution::find_median_sorted_arrays(nums1, nums2), expected);
    }

    #[test]
    fn test_004_6() {
        let nums1 = vec![1, 1, 1];
        let nums2 = vec![1, 1, 1];
        let expected = 1.0;
        assert_eq!(Solution::find_median_sorted_arrays(nums1, nums2), expected);
    }

    #[test]
    fn test_004_7() {
        let nums1 = vec![1, 2, 3, 4, 5];
        let nums2 = vec![6, 7, 8, 9, 10];
        let expected = 5.5;
        assert_eq!(Solution::find_median_sorted_arrays(nums1, nums2), expected);
    }

    #[test]
    fn test_004_8() {
        let nums1 = vec![1, 3, 5, 7, 9];
        let nums2 = vec![2, 4, 6, 8, 10];
        let expected = 5.5;
        assert_eq!(Solution::find_median_sorted_arrays(nums1, nums2), expected);
    }

    #[test]
    fn test_004_9() {
        let nums1 = vec![1, 3, 8];
        let nums2 = vec![7, 9, 10, 11];
        let expected = 8.0;
        assert_eq!(Solution::find_median_sorted_arrays(nums1, nums2), expected);
    }

    #[test]
    fn test_004_10() {
        let nums1 = vec![1, 2, 3];
        let nums2 = vec![4, 5, 6, 7, 8, 9];
        let expected = 5.0;
        assert_eq!(Solution::find_median_sorted_arrays(nums1, nums2), expected);
    }

    #[test]
    fn test_004_11() {
        let nums1 = vec![10];
        let nums2 = vec![1, 2, 3, 4, 5];
        let expected = 3.5;
        assert_eq!(Solution::find_median_sorted_arrays(nums1, nums2), expected);
    }

    #[test]
    fn test_004_12() {
        let nums1 = vec![1, 5, 9];
        let nums2 = vec![2, 3, 4, 6, 7, 8];
        let expected = 5.0;
        assert_eq!(Solution::find_median_sorted_arrays(nums1, nums2), expected);
    }

    #[test]
    fn test_004_13() {
        let nums1 = vec![1, 2, 3, 4];
        let nums2 = vec![5, 6, 7, 8];
        let expected = 4.5;
        assert_eq!(Solution::find_median_sorted_arrays(nums1, nums2), expected);
    }

    #[test]
    fn test_004_14() {
        let nums1 = vec![1, 3, 5, 7];
        let nums2 = vec![2, 4, 6, 8];
        let expected = 4.5;
        assert_eq!(Solution::find_median_sorted_arrays(nums1, nums2), expected);
    }

    #[test]
    fn test_004_15() {
        let nums1 = vec![1, 2, 3, 4, 5, 6];
        let nums2 = vec![];
        let expected = 3.5;
        assert_eq!(Solution::find_median_sorted_arrays(nums1, nums2), expected);
    }

    #[test]
    fn test_004_16() {
        let nums1 = vec![1, 2, 3, 4, 5];
        let nums2 = vec![];
        let expected = 3.0;
        assert_eq!(Solution::find_median_sorted_arrays(nums1, nums2), expected);
    }

    #[test]
    fn test_004_17() {
        let nums1 = vec![1, 3];
        let nums2 = vec![2, 4, 5, 6];
        let expected = 3.5;
        assert_eq!(Solution::find_median_sorted_arrays(nums1, nums2), expected);
    }

    #[test]
    fn test_004_18() {
        let nums1 = vec![1, 2, 3];
        let nums2 = vec![1, 2, 3];
        let expected = 2.0;
        assert_eq!(Solution::find_median_sorted_arrays(nums1, nums2), expected);
    }

    #[test]
    fn test_004_19() {
        let nums1 = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let nums2 = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let expected = 4.5;
        assert_eq!(Solution::find_median_sorted_arrays(nums1, nums2), expected);
    }

    #[test]
    fn test_004_20() {
        let nums1 = vec![1, 3, 5, 7, 9, 11];
        let nums2 = vec![2, 4, 6, 8, 10, 12];
        let expected = 6.5;
        assert_eq!(Solution::find_median_sorted_arrays(nums1, nums2), expected);
    }
}
