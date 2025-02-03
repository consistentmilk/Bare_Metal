pub enum State {
    Start,
    Increasing,
    Decreasing,
    Equal,
}

pub struct Solution;

impl Solution {
    pub fn longest_monotonic_subarray(nums: Vec<i32>) -> i32 {
        let n: usize = nums.len();

        if n == 1 {
            return 1;
        }

        let mut ptr: usize = 1;
        let mut max_long: usize = 1;
        let mut state: State = State::Start;

        loop {
            if ptr >= n {
                break;
            }

            match state {
                State::Start => {
                    if nums[ptr - 1] > nums[ptr] {
                        state = State::Decreasing;
                    } else if nums[ptr - 1] < nums[ptr] {
                        state = State::Increasing;
                    } else {
                        state = State::Equal;
                    }
                }

                State::Increasing => {
                    let tmp = ptr;

                    loop {
                        if nums[ptr - 1] > nums[ptr] {
                            state = State::Decreasing;
                            break;
                        } else if nums[ptr - 1] < nums[ptr] {
                            state = State::Increasing;
                            ptr += 1;
                        } else {
                            state = State::Equal;
                            break;
                        }

                        if ptr >= n {
                            break;
                        }
                    }

                    max_long = std::cmp::max(max_long, ptr - tmp + 1);
                }

                State::Decreasing => {
                    let tmp = ptr;

                    loop {
                        if nums[ptr - 1] > nums[ptr] {
                            state = State::Decreasing;
                            ptr += 1;
                        } else if nums[ptr - 1] < nums[ptr] {
                            state = State::Increasing;
                            break;
                        } else {
                            state = State::Equal;
                            break;
                        }

                        if ptr >= n {
                            break;
                        }
                    }

                    max_long = std::cmp::max(max_long, ptr - tmp + 1);
                }

                State::Equal => loop {
                    if nums[ptr - 1] > nums[ptr] {
                        state = State::Decreasing;
                        break;
                    } else if nums[ptr - 1] < nums[ptr] {
                        state = State::Increasing;
                        break;
                    } else {
                        ptr += 1;
                    }

                    if ptr >= n {
                        break;
                    }
                },
            }
        }

        max_long as i32
    }
}

pub struct SolutionAlt;

impl SolutionAlt {
    pub fn longest_monotonic_subarray(nums: Vec<i32>) -> i32 {
        nums[1..]
            .into_iter()
            .scan((0, nums[0]), |(sign, prev), &x| {
                if x > *prev {
                    *prev = x;

                    if *sign > 0 {
                        *sign += 1;
                    } else {
                        *sign = 2;
                    }

                    Some(*sign)
                } else if x < *prev {
                    *prev = x;

                    if *sign < 0 {
                        *sign -= 1;
                    } else {
                        *sign = -2;
                    }

                    Some(-(*sign))
                } else {
                    *sign = 0;

                    Some(1)
                }
            })
            .max()
            .unwrap_or(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3105_1() {
        let nums = vec![1, 4, 3, 3, 2];
        let expected = 2;

        assert_eq!(expected, Solution::longest_monotonic_subarray(nums));
    }

    #[test]
    fn test_3105_2() {
        let nums = vec![3, 3, 3, 3];
        let expected = 1;

        assert_eq!(expected, Solution::longest_monotonic_subarray(nums));
    }

    #[test]
    fn test_3105_3() {
        let nums = vec![3, 2, 1];
        let expected = 3;

        assert_eq!(expected, Solution::longest_monotonic_subarray(nums));
    }
}
