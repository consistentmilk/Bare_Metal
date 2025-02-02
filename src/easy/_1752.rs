pub struct Solution;

impl Solution {
    pub fn check(nums: Vec<i32>) -> bool {
        let n: usize = nums.len();

        if n == 1 {
            return true;
        }

        let mut ptr: usize = 1;

        loop {
            if ptr >= n {
                return true;
            }

            if nums[ptr - 1] <= nums[ptr] {
                ptr += 1;
            } else {
                ptr += 1;
                break;
            }
        }

        loop {
            if ptr >= n {
                break;
            }

            if nums[ptr - 1] <= nums[ptr] {
                ptr += 1;
            } else {
                return false;
            }
        }

        nums[0] >= nums[n - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1752_1() {
        let nums: Vec<i32> = vec![3, 4, 5, 1, 2];
        assert!(Solution::check(nums));
    }

    #[test]
    fn test_1752_2() {
        let nums: Vec<i32> = vec![2, 1, 3, 4];
        assert!(!Solution::check(nums));
    }

    #[test]
    fn test_1752_3() {
        let nums: Vec<i32> = vec![1, 2, 3];
        assert!(Solution::check(nums));
    }

    #[test]
    fn test_1752_4() {
        let nums: Vec<i32> = vec![6, 10, 6];
        assert!(Solution::check(nums));
    }
}
