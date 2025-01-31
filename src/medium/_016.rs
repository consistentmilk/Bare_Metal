pub struct Solution;

impl Solution {
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        // Sort the array to enable the two-pointer technique.
        // Example: If nums = [-1, 2, 1, -4], after sorting it becomes [-4, -1, 1, 2].
        nums.sort();

        let n: usize = nums.len();
        let mut closest_diff: i32 = i32::MAX; // Tracks the smallest difference between a triplet sum and the target.
        let mut closest_sum: i32 = i32::MAX; // Tracks the sum of the triplet with the smallest difference.

        // Iterate through the array, fixing the first element of the triplet.
        for i in 0..n - 2 {
            // If there are fewer than 3 elements left, break out of the loop.
            if i + 2 >= n {
                break;
            }

            // Calculate the minimal possible sum for the current triplet.
            // Example: If nums = [-4, -1, 1, 2], and i = 0, min_sum = -4 + (-1) + 1 = -4.
            let min_sum: i32 = nums[i] + nums[i + 1] + nums[i + 2];

            // If the minimal sum is already greater than the target, no need to check further.
            // Update the closest sum if necessary and break.
            if min_sum > target {
                let curr_diff: i32 = (min_sum - target).abs();

                if curr_diff < closest_diff {
                    closest_sum = min_sum;
                }

                break;
            }

            // Calculate the maximal possible sum for the current triplet.
            // Example: If nums = [-4, -1, 1, 2], and i = 0, max_sum = -4 + 1 + 2 = -1.
            let max_sum: i32 = nums[i] + nums[n - 2] + nums[n - 1];

            // If the maximal sum is less than the target, the closest sum for this triplet is the maximal sum.
            // Update the closest sum if necessary and continue to the next iteration.
            if max_sum < target {
                let curr_diff: i32 = (max_sum - target).abs();

                if curr_diff < closest_diff {
                    closest_diff = curr_diff;
                    closest_sum = max_sum;
                }

                continue;
            }

            // Use two pointers to find the closest sum for the current triplet.
            let mut j: usize = i + 1; // Pointer to the second element of the triplet.
            let mut k: usize = n - 1; // Pointer to the third element of the triplet.

            while j < k {
                // Calculate the current sum of the triplet.
                // Example: If nums = [-4, -1, 1, 2], i = 0, j = 1, k = 3, curr_sum = -4 + (-1) + 2 = -3.
                let curr_sum: i32 = nums[i] + nums[j] + nums[k];

                // If the current sum is greater than the target, move the right pointer (k) to the left.
                if curr_sum > target {
                    k -= 1;

                    // Skip duplicate elements to avoid redundant calculations.
                    while j < k && nums[k] == nums[k + 1] {
                        k -= 1;
                    }
                }
                // If the current sum is less than the target, move the left pointer (j) to the right.
                else if curr_sum < target {
                    j += 1;

                    // Skip duplicate elements to avoid redundant calculations.
                    while j < k && nums[j] == nums[j - 1] {
                        j += 1;
                    }
                }
                // If the current sum equals the target, return the target immediately.
                else {
                    return target;
                }

                // Calculate the difference between the current sum and the target.
                let curr_diff: i32 = (curr_sum - target).abs();

                // Update the closest sum and difference if the current sum is closer to the target.
                if curr_diff < closest_diff {
                    closest_diff = curr_diff;
                    closest_sum = curr_sum;
                }
            }
        }

        // Return the closest sum found.
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
