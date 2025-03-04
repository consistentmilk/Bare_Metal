pub struct Solution;

impl Solution {
    ///
    /// [1,2,2,1,1,0]
    /// [1,4,2,0,0,0]
    ///
    ///
    /// If nums[i] == nums[i + 1], then multiply nums[i] by 2 and set nums[i + 1] to 0. Otherwise, you skip this operation.
    ///
    pub fn apply_operations(mut nums: Vec<i32>) -> Vec<i32> {
        let n: usize = nums.len();
        let mut res: Vec<i32> = Vec::with_capacity(n);
        let mut ptr: usize = 1;
        let mut zeros: usize = 0;

        if n == 1 {
            return nums;
        }

        loop {
            if ptr >= n {
                break;
            }

            if nums[ptr - 1] == nums[ptr] {
                nums[ptr - 1] = nums[ptr - 1] * 2;
                nums[ptr] = 0;
            }

            if nums[ptr - 1] == 0 {
                zeros += 1;
            } else {
                res.push(nums[ptr - 1]);
            }

            ptr += 1;
        }

        if nums[n - 1] == 0 {
            zeros += 1;
        } else {
            res.push(nums[n - 1]);
        }

        for _ in 0..zeros {
            res.push(0);
        }

        res
    }

    pub fn apply_operations_optimized(mut nums: Vec<i32>) -> Vec<i32> {
        let n: usize = nums.len();
        let mut write_idx: usize = 0;

        for i in 0..n {
            if i < n - 1 && nums[i] == nums[i + 1] && nums[i] != 0 {
                nums[i] *= 2;
                nums[i + 1] = 0;
            }

            if nums[i] != 0 {
                if i != write_idx {
                    let tmp: i32 = nums[i];
                    nums[i] = nums[write_idx];
                    nums[write_idx] = tmp;
                }

                write_idx += 1;
            }
        }

        nums
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2460_1() {
        let nums: Vec<i32> = vec![1, 2, 2, 1, 1, 0];
        let expected: Vec<i32> = vec![1, 4, 2, 0, 0, 0];

        let res: Vec<i32> = Solution::apply_operations(nums.clone());
        assert_eq!(expected, res);

        let res: Vec<i32> = Solution::apply_operations_optimized(nums.clone());
        assert_eq!(expected, res);
    }

    #[test]
    fn test_2460_2() {
        let nums: Vec<i32> = vec![0, 1];
        let expected: Vec<i32> = vec![1, 0];

        let res: Vec<i32> = Solution::apply_operations(nums.clone());
        assert_eq!(expected, res);

        let res: Vec<i32> = Solution::apply_operations_optimized(nums.clone());
        assert_eq!(expected, res);
    }
}
