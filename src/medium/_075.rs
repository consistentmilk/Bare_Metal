/***********************************************************************
Leetcode 75 - Sort Colors

Intuition
---------
Use a one-pass, in-place sorting algorithm (Dutch National Flag) with
three pointers to classify the array into three partitions:
- Left: all 0s (red)
- Middle: all 1s (white)
- Right: all 2s (blue)

Algorithm
---------
1. Initialize three pointers:
   - low: position for the next 0
   - mid: current element to evaluate
   - high: position for the next 2
2. Traverse the array with `mid` until it passes `high`:
   - If nums[mid] == 0: swap with nums[low], advance both low and mid
   - If nums[mid] == 1: mid++
   - If nums[mid] == 2: swap with nums[high], decrease high

Time Complexity
---------------
- O(n): Each element is checked at most once

Space Complexity
----------------
- O(1): Sorting is done in-place
***********************************************************************/
pub struct Solution;

impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        // Early return if vector is empty
        if nums.is_empty() {
            return;
        }

        // Pointer to the next position to place 0 (red)
        let mut low: usize = 0;

        // Pointer to the current index under evaluation
        let mut mid: usize = 0;

        // Pointer to the next position to place 2 (blue)
        let mut high: usize = nums.len() - 1;

        // Traverse while mid <= high
        while mid <= high {
            match nums[mid] {
                0 => {
                    // Swap current value with low position if 0 (red)
                    nums.swap(mid, low);

                    // Move low and mid forward
                    low += 1;
                    mid += 1;
                }

                1 => {
                    // Leave 1s (white) in the middle and move on
                    mid += 1;
                }

                2 => {
                    // Swap current value with high position if 2 (blue)
                    nums.swap(mid, high);

                    // Decrease high pointer; recheck swapped element
                    if high == 0 {
                        break; // prevent underflow
                    }

                    high -= 1;
                }

                _ => {
                    // Invalid input, but not expected as per constraints
                    panic!("Unexpected value found in nums");
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_75_example_case_01() {
        // Test sorting a mixed vector of 0s, 1s, and 2s
        let mut nums: Vec<i32> = vec![2, 0, 2, 1, 1, 0];
        Solution::sort_colors(&mut nums);
        assert_eq!(nums, vec![0, 0, 1, 1, 2, 2]);
    }

    #[test]
    fn test_75_two_each_02() {
        // Contains equal number of each color
        let mut nums: Vec<i32> = vec![1, 2, 0, 0, 2, 1];
        Solution::sort_colors(&mut nums);
        assert_eq!(nums, vec![0, 0, 1, 1, 2, 2]);
    }

    #[test]
    fn test_75_already_sorted_03() {
        // Already sorted input
        let mut nums: Vec<i32> = vec![0, 0, 1, 1, 2, 2];
        Solution::sort_colors(&mut nums);
        assert_eq!(nums, vec![0, 0, 1, 1, 2, 2]);
    }

    #[test]
    fn test_75_reverse_sorted_04() {
        // Reverse sorted input
        let mut nums: Vec<i32> = vec![2, 2, 1, 1, 0, 0];
        Solution::sort_colors(&mut nums);
        assert_eq!(nums, vec![0, 0, 1, 1, 2, 2]);
    }

    #[test]
    fn test_75_all_zeros_05() {
        // All elements are the same (0)
        let mut nums: Vec<i32> = vec![0, 0, 0];
        Solution::sort_colors(&mut nums);
        assert_eq!(nums, vec![0, 0, 0]);
    }

    #[test]
    fn test_75_all_ones_06() {
        // All elements are the same (1)
        let mut nums: Vec<i32> = vec![1, 1, 1];
        Solution::sort_colors(&mut nums);
        assert_eq!(nums, vec![1, 1, 1]);
    }

    #[test]
    fn test_75_all_twos_07() {
        // All elements are the same (2)
        let mut nums: Vec<i32> = vec![2, 2, 2];
        Solution::sort_colors(&mut nums);
        assert_eq!(nums, vec![2, 2, 2]);
    }

    #[test]
    fn test_75_single_element_08() {
        // Test with only one element
        let mut nums: Vec<i32> = vec![1];
        Solution::sort_colors(&mut nums);
        assert_eq!(nums, vec![1]);
    }

    #[test]
    fn test_75_two_elements_unsorted_09() {
        // Two elements out of order
        let mut nums: Vec<i32> = vec![2, 0];
        Solution::sort_colors(&mut nums);
        assert_eq!(nums, vec![0, 2]);
    }

    #[test]
    fn test_75_empty_vector_10() {
        // Empty input vector
        let mut nums: Vec<i32> = vec![];
        Solution::sort_colors(&mut nums);
        assert_eq!(nums, vec![]);
    }
}
