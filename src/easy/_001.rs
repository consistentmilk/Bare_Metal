use std::collections::HashMap;

pub struct Solution;

impl Solution {
    ///
    /// Problem 1: Two Sum
    /// Description: Given an array of integers and a target integer, find the indices of two integers in the array
    /// such that their sum adds up to the target.
    ///
    /// Input: nums: Vec<i32>, target: i32
    /// Constraints:
    /// 1. nums.len() >= 2
    /// 2. 10^(-9) <= nums[i] <= 10^(9), where 0 <= i <= nums.len()
    /// 3. 10^(-9) <= target <= 10^(9)
    /// 4. It is guaranteed that a solution, if exists, is unique.
    ///     - For example, nums = [2, 3, 1, 4], target = 5, is not a valid input.
    ///       As nums[0] + nums[1] = 2 + 3 = 5 and nums[2] + nums[3] = 5.
    ///
    /// Example: nums = [2, 7, 11, 15], target = 9, solution = [0, 1]
    /// Example: nums = [3, 3], target = 6, solution = [0, 1]
    /// Example: nums = [3, 2, 4], target = 6, solution = [1, 2]
    ///
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();

        for (idx, val) in nums.iter().enumerate() {
            let complement: i32 = target - val;

            if map.contains_key(&complement) {
                return vec![map[&complement], idx as i32];
            }

            map.insert(*val, idx as i32);
        }

        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_001_1() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1])
    }

    #[test]
    fn test_001_2() {
        assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2])
    }
}
