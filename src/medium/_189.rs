pub struct Solution {}

// Given an integer array nums, rotate the array to the right by k steps, where k is non-negative.

// Example 1:
// Input: nums = [1,2,3,4,5,6,7], k = 3
// Output: [5,6,7,1,2,3,4]
// Explanation:
// rotate 1 steps to the right: [7,1,2,3,4,5,6]
// rotate 2 steps to the right: [6,7,1,2,3,4,5]
// rotate 3 steps to the right: [5,6,7,1,2,3,4]

// Example 2:
// Input: nums = [-1,-100,3,99], k = 2
// Output: [3,99,-1,-100]
// Explanation:
// rotate 1 steps to the right: [99,-1,-100,3]
// rotate 2 steps to the right: [3,99,-1,-100]

// Constraints:
// 1 <= nums.length <= 105
// -231 <= nums[i] <= 231 - 1
// 0 <= k <= 105

// Follow up:
// Try to come up with as many solutions as you can. There are at least three different ways to solve this problem.
// Could you do it in-place with O(1) extra space?

impl Solution {
    pub fn rotate_v1(nums: &mut Vec<i32>, k: i32) {
        let n: usize = nums.len();
        let k: usize = k as usize;

        if n == 0 || k <= 0 {
            return;
        }

        let nums_copy: Vec<i32> = nums.clone();

        for i in 0..n {
            nums[(i + k) % n] = nums_copy[i];
        }
    }

    pub fn rotate_v2(nums: &mut Vec<i32>, k: i32) {
        let k: usize = k as usize % nums.len();

        nums.reverse();
        nums[0..k].reverse();
        nums[k..].reverse();
    }

    pub fn rotate_unsafe_incomplete(nums: &mut Vec<i32>, k: i32) {
        use std::ops::Range;

        let k: usize = k as usize % nums.len();
        let Range { start, end } = nums.as_mut_ptr_range();

        let (front_rotate, back_rotate) = unsafe {
            (
                std::slice::from_raw_parts_mut(start, k),
                std::slice::from_raw_parts_mut(end.sub(k), k),
            )
        };

        revswap(front_rotate, back_rotate, k);

        #[inline]
        fn revswap(a: &mut [i32], b: &mut [i32], n: usize) {
            let (a, b) = (&mut a[..n], &mut b[..n]);

            let mut i = 0;
            while i < n {
                std::mem::swap(&mut a[i], &mut b[n - 1 - i]);
                i += 1;
            }
        }
    }
}
