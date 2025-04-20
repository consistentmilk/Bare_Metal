/*
Intuition:
1. We treat each subset as a bitmask from 0 to 2ⁿ−1, where bit j indicates inclusion of nums[j].
2. To avoid duplicate subsets when nums contains repeats, we skip any mask where a duplicate element
   is selected (bit j = 1) but its identical predecessor (j−1) is not selected (bit j−1 = 0).
3. This bitwise filter ensures each unique combination of values is generated exactly once.

Algorithm:
1. Sort `nums` so equal values are adjacent.
2. Let n = nums.len().
3. For each mask in 0..(1 << n):
   a. Check validity: for any j in 1..n, if nums[j] == nums[j−1] and (mask>>j)&1 == 1 and (mask>>(j−1))&1 == 0, skip mask.
   b. Otherwise, build subset by collecting nums[j] for each j where (mask>>j)&1 == 1.
   c. Push the resulting subset into `result`.
4. Return `result`.

Time Complexity:
- O(2ⁿ · n): we examine 2ⁿ masks and for each, up to n bit checks and subset construction.

Space Complexity:
- O(n · 2ⁿ): to store all subsets (output), plus O(n) for each temporary subset.
*/

pub struct Solution;
// Namespace for the solution method.

impl Solution {
    // Generate all unique subsets using bit manipulation and duplicate-skipping mask check.
    pub fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        // Clone and sort the input to group duplicates together.
        nums.sort_unstable();

        // Prepare the result vector to collect subsets.
        let mut result: Vec<Vec<i32>> = Vec::new();

        // Number of elements.
        let n: usize = nums.len();

        // Total number of possible bitmasks = 2^n.
        let total_masks: usize = 1 << n;

        // Iterate through every bitmask from 0 to 2^n − 1.
        for mask in 0..total_masks {
            // Flag to indicate if this mask should be skipped due to duplicate rule.
            let mut skip: bool = false;

            // Check each position j for the duplicate-skip condition.
            for j in 1..n {
                // If current and previous values are equal...
                if nums[j] == nums[j - 1] {
                    // Extract bit j and bit j−1 from the mask.
                    let bit_j: usize = (mask >> j) & 1;
                    let bit_prev: usize = (mask >> (j - 1)) & 1;

                    // If we include the later duplicate but exclude the earlier, skip.
                    if bit_j == 1 && bit_prev == 0 {
                        skip = true;
                        break;
                    }
                }
            }

            // If mask is invalid, proceed to the next mask.
            if skip {
                continue;
            }

            // Build the subset corresponding to this valid mask.
            let mut subset: Vec<i32> = Vec::new();

            for j in 0..n {
                // If bit j is set, include nums[j].
                if ((mask >> j) & 1) == 1 {
                    subset.push(nums[j]);
                }
            }

            // Add the constructed subset to the results.
            result.push(subset);
        }

        // Return all unique subsets.
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_090_single_element() {
        // Input with a single element
        let nums: Vec<i32> = vec![0];
        // Generate all unique subsets using bitwise approach
        let mut result = Solution::subsets_with_dup(nums);
        // Define the expected subsets
        let mut expected: Vec<Vec<i32>> = vec![vec![], vec![0]];
        // Sort both result and expected for order‑independent comparison
        result.sort();
        expected.sort();
        // Verify that generated subsets match expected
        assert_eq!(result, expected);
    }

    #[test]
    fn test_090_all_distinct() {
        // Input with all distinct elements
        let nums: Vec<i32> = vec![1, 2, 3];
        // Generate subsets
        let mut result = Solution::subsets_with_dup(nums);
        // Expected power set of [1,2,3]
        let mut expected: Vec<Vec<i32>> = vec![
            vec![],
            vec![1],
            vec![2],
            vec![3],
            vec![1, 2],
            vec![1, 3],
            vec![2, 3],
            vec![1, 2, 3],
        ];
        // Sort both to ignore ordering
        result.sort();
        expected.sort();
        // Assert equality
        assert_eq!(result, expected);
    }

    #[test]
    fn test_090_one_duplicate() {
        // Input containing one duplicate
        let nums: Vec<i32> = vec![1, 2, 2];
        // Generate subsets
        let mut result = Solution::subsets_with_dup(nums);
        // Expected unique subsets for [1,2,2]
        let mut expected: Vec<Vec<i32>> = vec![
            vec![],
            vec![1],
            vec![2],
            vec![1, 2],
            vec![2, 2],
            vec![1, 2, 2],
        ];
        // Sort for comparison
        result.sort();
        expected.sort();
        // Check correctness
        assert_eq!(result, expected);
    }

    #[test]
    fn test_090_all_duplicates() {
        // Input where all elements are the same
        let nums: Vec<i32> = vec![2, 2, 2];
        // Generate subsets
        let mut result = Solution::subsets_with_dup(nums);
        // Expected subsets: [], [2], [2,2], [2,2,2]
        let mut expected: Vec<Vec<i32>> = vec![vec![], vec![2], vec![2, 2], vec![2, 2, 2]];
        // Sort to normalize order
        result.sort();
        expected.sort();
        // Assert the two lists match
        assert_eq!(result, expected);
    }

    #[test]
    fn test_090_negative_duplicates() {
        // Input with negative duplicates
        let nums: Vec<i32> = vec![-1, -1];
        // Generate subsets
        let mut result = Solution::subsets_with_dup(nums);
        // Expected subsets: [], [-1], [-1, -1]
        let mut expected: Vec<Vec<i32>> = vec![vec![], vec![-1], vec![-1, -1]];
        // Sort both lists
        result.sort();
        expected.sort();
        // Verify match
        assert_eq!(result, expected);
    }

    #[test]
    fn test_090_mixed_duplicates() {
        // Input with mixed duplicates and distinct
        let nums: Vec<i32> = vec![1, 2, 2, 3];
        // Generate subsets
        let mut result = Solution::subsets_with_dup(nums);
        // Expected subsets for [1,2,2,3]
        let mut expected: Vec<Vec<i32>> = vec![
            vec![],
            vec![1],
            vec![2],
            vec![3],
            vec![1, 2],
            vec![1, 3],
            vec![2, 2],
            vec![2, 3],
            vec![1, 2, 2],
            vec![1, 2, 3],
            vec![2, 2, 3],
            vec![1, 2, 2, 3],
        ];
        // Sort for comparison
        result.sort();
        expected.sort();
        // Assert generated subsets are correct
        assert_eq!(result, expected);
    }

    #[test]
    fn test_090_end_duplicates() {
        // Input with duplicates at the end
        let nums: Vec<i32> = vec![1, 1, 2];
        // Generate subsets
        let mut result = Solution::subsets_with_dup(nums);
        // Expected subsets for [1,1,2]
        let mut expected: Vec<Vec<i32>> = vec![
            vec![],
            vec![1],
            vec![2],
            vec![1, 1],
            vec![1, 2],
            vec![1, 1, 2],
        ];
        // Sort both lists
        result.sort();
        expected.sort();
        // Verify correctness
        assert_eq!(result, expected);
    }

    #[test]
    fn test_090_start_duplicates() {
        // Input with duplicates at the start
        let nums: Vec<i32> = vec![3, 3, 1];
        // Generate subsets
        let mut result = Solution::subsets_with_dup(nums);
        // Expected subsets for [3,3,1] -> sorted [1,3,3]
        let mut expected: Vec<Vec<i32>> = vec![
            vec![],
            vec![1],
            vec![3],
            vec![1, 3],
            vec![3, 3],
            vec![1, 3, 3],
        ];
        // Sort to standardize order
        result.sort();
        expected.sort();
        // Assert they match
        assert_eq!(result, expected);
    }

    #[test]
    fn test_090_zero_value_duplicates() {
        // Input containing zero and duplicates
        let nums: Vec<i32> = vec![0, 1, 0];
        // Generate subsets
        let mut result = Solution::subsets_with_dup(nums);
        // Expected subsets for [0,1,0] -> sorted [0,0,1]
        let mut expected: Vec<Vec<i32>> = vec![
            vec![],
            vec![0],
            vec![1],
            vec![0, 0],
            vec![0, 1],
            vec![0, 0, 1],
        ];
        // Sort both for comparison
        result.sort();
        expected.sort();
        // Verify result
        assert_eq!(result, expected);
    }

    #[test]
    fn test_090_large_distinct_length() {
        // Input with four distinct elements
        let nums: Vec<i32> = vec![1, 2, 3, 4];
        // Generate subsets
        let result = Solution::subsets_with_dup(nums);
        // Verify the total number of subsets is 16 (2^4)
        let len = result.len();
        // Assert the correct count
        assert_eq!(len, 16);
    }
}
