pub struct Solution;

impl Solution {
    pub fn max_rotate_function(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let (s, f) = nums
            .iter()
            .enumerate()
            .fold((0, 0), |(s, f), (i, &x)| (s + x, f + x * i as i32));

        nums.into_iter()
            .scan(f, |f, x| {
                *f += -s + (n as i32 * x);

                Some(*f)
            })
            .max()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_396_1() {
        let arr: Vec<i32> = vec![4, 3, 2, 6];
        let expected: i32 = 26;

        assert_eq!(Solution::max_rotate_function(arr), expected);
    }

    #[test]
    fn test_396_2() {
        let arr: Vec<i32> = vec![100];
        let expected: i32 = 0;

        assert_eq!(Solution::max_rotate_function(arr), expected);
    }
}
