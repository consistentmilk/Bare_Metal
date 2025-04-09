use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut set: HashSet<i32> = HashSet::new();

        for num in nums {
            if num < k {
                return -1;
            }

            if set.contains(&num) {
                continue;
            } else if num > k {
                set.insert(num);
            }
        }

        set.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3375_1() {
        let nums = vec![1, 2, 3];
        let k = 2;
        let expected = -1;
        let result = Solution::min_operations(nums, k);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_3375_2() {
        let nums = vec![2, 2, 2];
        let k = 2;
        let expected = 0;
        let result = Solution::min_operations(nums, k);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_3375_3() {
        let nums = vec![3, 4, 5];
        let k = 2;
        let expected = 3;
        let result = Solution::min_operations(nums, k);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_3375_4() {
        let nums = vec![4, 4, 4, 5, 5];
        let k = 4;
        let expected = 1;
        let result = Solution::min_operations(nums, k);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_3375_5() {
        let nums = vec![5, 6, 7, 8];
        let k = 5;
        let expected = 3;
        let result = Solution::min_operations(nums, k);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_3375_6() {
        let nums = vec![10, 10, 10, 10];
        let k = 10;
        let expected = 0;
        let result = Solution::min_operations(nums, k);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_3375_7() {
        let nums = vec![1, 1, 1, 1];
        let k = 2;
        let expected = -1;
        let result = Solution::min_operations(nums, k);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_3375_8() {
        let nums = vec![3, 3, 4, 4, 5, 5];
        let k = 3;
        let expected = 2;
        let result = Solution::min_operations(nums, k);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_3375_9() {
        let nums = vec![];
        let k = 5;
        let expected = 0;
        let result = Solution::min_operations(nums, k);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_3375_10() {
        let nums = vec![6, 7, 8, 9, 10];
        let k = 5;
        let expected = 5;
        let result = Solution::min_operations(nums, k);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_3375_11() {
        let nums = vec![5, 5, 6, 6, 7, 7];
        let k = 5;
        let expected = 2;
        let result = Solution::min_operations(nums, k);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_3375_12() {
        let nums = vec![100, 200, 300];
        let k = 50;
        let expected = 3;
        let result = Solution::min_operations(nums, k);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_3375_13() {
        let nums = vec![2, 3, 4, 5];
        let k = 1;
        let expected = 4;
        let result = Solution::min_operations(nums, k);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_3375_14() {
        let nums = vec![5];
        let k = 5;
        let expected = 0;
        let result = Solution::min_operations(nums, k);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_3375_15() {
        let nums = vec![6];
        let k = 5;
        let expected = 1;
        let result = Solution::min_operations(nums, k);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_3375_16() {
        let nums = vec![4];
        let k = 5;
        let expected = -1;
        let result = Solution::min_operations(nums, k);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_3375_17() {
        let nums = vec![5, 6, 7, 8, 9, 10, 11];
        let k = 5;
        let expected = 6;
        let result = Solution::min_operations(nums, k);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_3375_18() {
        let nums = vec![5, 5, 5, 6, 6, 7, 7, 8, 8];
        let k = 5;
        // Expected: Distinct numbers >5 are 6,7,8 → count=3
        let expected = 3;
        let result = Solution::min_operations(nums, k);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_3375_19() {
        let nums = vec![10, 10, 20, 20, 30, 30];
        let k = 15;
        let expected = -1; // Because 10 < 15 → invalid
        let result = Solution::min_operations(nums, k);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_3375_20() {
        let nums = vec![1, 2, 3, 4, 5];
        let k = 3;
        let expected = -1;
        let result = Solution::min_operations(nums, k);
        assert_eq!(result, expected);
    }
}
