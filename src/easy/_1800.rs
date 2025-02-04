pub struct Solution;

impl Solution {
    pub fn max_ascending_sum(nums: Vec<i32>) -> i32 {
        let n: usize = nums.len();

        if n == 1 {
            return nums[0];
        }

        let mut ptr: usize = 1;
        let mut max_sum: i32 = 0;

        loop {
            if ptr >= n {
                break;
            }

            let mut curr_sum: i32 = 0;
            while ptr < n && nums[ptr - 1] < nums[ptr] {
                curr_sum += nums[ptr - 1];
                ptr += 1;
            }

            curr_sum += nums[ptr - 1];

            if curr_sum > max_sum {
                max_sum = curr_sum;
            }

            ptr += 1;
        }

        max_sum
    }
}

pub struct SolutionAlt;

impl SolutionAlt {
    pub fn max_ascending_sum(nums: Vec<i32>) -> i32 {
        nums.into_iter()
            .fold((0, 0, None::<i32>), |(max, run, last), curr| {
                let Some(last) = last else {
                    return (curr, curr, Some(curr));
                };

                if last < curr {
                    let run = run + curr;
                    (max.max(run), run, Some(curr))
                } else {
                    let run = curr;
                    (max.max(run), run, Some(curr))
                }
            })
            .0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1800_1() {
        let nums = vec![10, 20, 30, 5, 10, 50];
        let expected = 65;

        assert_eq!(expected, Solution::max_ascending_sum(nums));
    }

    #[test]
    fn test_1800_2() {
        let nums = vec![10, 20, 30, 40, 50];
        let expected = 150;

        assert_eq!(expected, Solution::max_ascending_sum(nums));
    }

    #[test]
    fn test_1800_3() {
        let nums = vec![12, 17, 15, 13, 10, 11, 12];
        let expected = 33;

        assert_eq!(expected, Solution::max_ascending_sum(nums));
    }
}
