pub struct Solution;

impl Solution {
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
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
