pub struct Solution;

impl Solution {
    pub fn pivot_array(nums: Vec<i32>, pivot: i32) -> Vec<i32> {
        let n: usize = nums.len();

        let mut res: Vec<i32> = vec![0; n];
        let mut lesser_idx: usize = 0;
        let mut greater_idx: usize = n - 1;

        for (i, j) in (0..n).zip((0..n).rev()) {
            if nums[i] < pivot {
                res[lesser_idx] = nums[i];
                lesser_idx += 1;
            }

            if nums[j] > pivot {
                res[greater_idx] = nums[j];
                greater_idx -= 1;
            }
        }

        while lesser_idx <= greater_idx {
            res[lesser_idx] = pivot;
            lesser_idx += 1;
        }

        res
    }

    pub fn pivot_array_std(mut nums: Vec<i32>, pivot: i32) -> Vec<i32> {
        nums.sort_by_key(|&num| {
            if num < pivot {
                0
            } else if num == pivot {
                1
            } else {
                2
            }
        });

        nums
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2161_1() {
        let nums: Vec<i32> = vec![9, 12, 5, 10, 14, 3, 10];
        let pivot: i32 = 10;
        let expected: Vec<i32> = vec![9, 5, 3, 10, 10, 12, 14];
        let res: Vec<i32> = Solution::pivot_array(nums, pivot);

        assert_eq!(expected, res);
    }

    #[test]
    fn test_2161_2() {
        let nums: Vec<i32> = vec![-3, 4, 3, 2];
        let pivot: i32 = 2;
        let expected: Vec<i32> = vec![-3, 2, 4, 3];
        let res: Vec<i32> = Solution::pivot_array(nums, pivot);

        assert_eq!(expected, res);
    }
}
