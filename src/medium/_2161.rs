pub struct Solution;

impl Solution {
    pub fn pivot_array(mut nums: Vec<i32>, pivot: i32) -> Vec<i32> {
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
