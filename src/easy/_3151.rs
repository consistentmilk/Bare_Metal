pub struct Solution;

impl Solution {
    pub fn is_array_special(nums: Vec<i32>) -> bool {
        let n: usize = nums.len();
        let mut ptr: usize = 1;

        loop {
            if ptr >= n {
                break;
            }

            if (nums[ptr - 1] + nums[ptr]) % 2 == 0 {
                return false;
            }

            ptr += 1;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3151_1() {
        let nums = vec![1];
        assert!(Solution::is_array_special(nums));
    }

    #[test]
    fn test_3151_2() {
        let nums = vec![2, 1, 4];
        assert!(Solution::is_array_special(nums));
    }

    #[test]
    fn test_3151_3() {
        let nums = vec![4, 3, 1, 6];
        assert!(!Solution::is_array_special(nums));
    }
}
