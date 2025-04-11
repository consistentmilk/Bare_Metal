use std::cmp::Ordering;

pub struct Solution;

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort_unstable();

        let n: usize = nums.len();
        let mut res_vec: Vec<Vec<i32>> = Vec::new();

        for i in 0..n {
            // Skip duplicates for i
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }

            if nums[i] > 0 {
                break;
            }

            let mut j: usize = i + 1;
            let mut k: usize = n - 1;

            while j < k {
                let sum: i32 = nums[i] + nums[j] + nums[k];

                match sum.cmp(&0) {
                    Ordering::Greater => k -= 1,

                    Ordering::Less => j += 1,

                    Ordering::Equal => {
                        res_vec.push(vec![nums[i], nums[j], nums[k]]);
                        j += 1;
                        k -= 1;

                        // Skip duplicates for j
                        while j < k && nums[j] == nums[j - 1] {
                            j += 1;
                        }

                        // Skip duplicates for k
                        while j < k && nums[k] == nums[k + 1] {
                            k -= 1;
                        }
                    }
                }
            }
        }

        res_vec
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_015_1() {
        // Basic case with one valid triplet
        let nums = vec![-1, 0, 1, 2, -1, -4];
        let result = Solution::three_sum(nums);
        let expected = vec![vec![-1, -1, 2], vec![-1, 0, 1]];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_015_2() {
        // No possible triplet
        let nums = vec![1, 2, 3, 4];
        let result = Solution::three_sum(nums);
        let expected: Vec<Vec<i32>> = vec![];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_015_3() {
        // All zeros
        let nums = vec![0, 0, 0, 0];
        let result = Solution::three_sum(nums);
        let expected = vec![vec![0, 0, 0]];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_015_4() {
        // Multiple duplicates of valid triplets
        let nums = vec![-1, -1, -1, 0, 1, 1, 1];
        let result = Solution::three_sum(nums);
        let expected = vec![vec![-1, 0, 1]];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_015_5() {
        // Empty input
        let nums = vec![];
        let result = Solution::three_sum(nums);
        let expected: Vec<Vec<i32>> = vec![];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_015_6() {
        // All positive numbers
        let nums = vec![1, 2, 3, 4, 5];
        let result = Solution::three_sum(nums);
        let expected: Vec<Vec<i32>> = vec![];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_015_7() {
        // All negative numbers
        let nums = vec![-5, -4, -3, -2, -1];
        let result = Solution::three_sum(nums);
        let expected: Vec<Vec<i32>> = vec![];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_015_8() {
        // Large input with multiple solutions
        let nums = vec![-4, -2, -2, -1, -1, 0, 1, 2, 2, 3, 4];
        let mut result = Solution::three_sum(nums);

        // All possible valid triplets (each sorted)
        let mut expected_triplets = vec![
            vec![-4, 0, 4],
            vec![-4, 1, 3],
            vec![-2, -2, 4],
            vec![-2, -1, 3],
            vec![-2, 0, 2],
            vec![-1, -1, 2],
            vec![-1, 0, 1],
            vec![-4, 2, 2],
        ];

        result.sort();
        expected_triplets.sort();

        assert_eq!(result, expected_triplets);
    }

    #[test]
    fn test_015_9() {
        // Exactly three elements that sum to zero
        let nums = vec![-5, 2, 3];
        let result = Solution::three_sum(nums);
        let expected = vec![vec![-5, 2, 3]];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_015_10() {
        // Edge case with minimal input (less than 3 elements)
        let nums = vec![0, 1];
        let result = Solution::three_sum(nums);
        let expected: Vec<Vec<i32>> = vec![];
        assert_eq!(result, expected);
    }
}
