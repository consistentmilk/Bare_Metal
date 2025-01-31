pub struct Solution;

impl Solution {
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort();

        let n: usize = nums.len();
        let mut closest_diff: i32 = i32::MAX;
        let mut closest_sum: i32 = i32::MAX;

        for i in 0..n - 2 {
            if i + 2 >= n {
                break;
            }

            let min_sum: i32 = nums[i] + nums[i + 1] + nums[i + 2];
            if min_sum > target {
                let curr_diff: i32 = (min_sum - target).abs();

                if curr_diff < closest_diff {
                    closest_diff = curr_diff;
                    closest_sum = min_sum;
                }

                break;
            }

            let max_sum: i32 = nums[i] + nums[n - 2] + nums[n - 1];
            if max_sum < target {
                let curr_diff: i32 = (max_sum - target).abs();

                if curr_diff < closest_diff {
                    closest_diff = curr_diff;
                    closest_sum = max_sum;
                }

                continue;
            }

            let mut j: usize = i + 1;
            let mut k: usize = n - 1;

            while j < k {
                let curr_sum: i32 = nums[i] + nums[j] + nums[k];

                if curr_sum > target {
                    k -= 1;

                    while j < k && nums[k] == nums[k + 1] {
                        k -= 1;
                    }
                } else if curr_sum < target {
                    j += 1;

                    while j < k && nums[j] == nums[j - 1] {
                        j += 1;
                    }
                } else {
                    return target;
                }

                let curr_diff: i32 = (curr_sum - target).abs();
                if curr_diff < closest_diff {
                    closest_diff = curr_diff;
                    closest_sum = curr_sum;
                }
            }
        }

        closest_sum
    }

    pub fn __three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort();

        let n: usize = nums.len();
        let mut closest_diff: i32 = i32::MAX;
        let mut closest_sum: i32 = i32::MAX;

        for i in 0..n - 2 {
            let mut j: usize = i + 1;
            let mut k: usize = n - 1;

            while j < k {
                let curr_sum: i32 = nums[i] + nums[j] + nums[k];

                if curr_sum > target {
                    k -= 1;
                } else if curr_sum < target {
                    j += 1;
                } else {
                    return target;
                }

                let curr_diff: i32 = (curr_sum - target).abs();

                if closest_diff > curr_diff {
                    closest_diff = curr_diff;
                    closest_sum = curr_sum;
                }
            }
        }

        closest_sum
    }

    pub fn _three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort_unstable();
        let mut closest_sum: i32 = nums[0] + nums[1] + nums[2];

        for i in 0..nums.len() - 2 {
            let mut left: usize = i + 1;
            let mut right: usize = nums.len() - 1;

            while left < right {
                let current_sum: i32 = nums[i] + nums[left] + nums[right];

                if (current_sum - target).abs() < (closest_sum - target).abs() {
                    closest_sum = current_sum;
                }

                if current_sum < target {
                    left += 1;
                } else if current_sum > target {
                    right -= 1;
                } else {
                    return current_sum;
                }
            }
        }

        closest_sum
    }
}
