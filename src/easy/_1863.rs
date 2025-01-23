pub struct Solution;

impl Solution {
    pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
        let mut result: i32 = 0;

        // Capture each bit that is set in any of the elements
        for &num in nums.iter() {
            result |= num;
        }

        // Multiply by the number of subset XOR totals that will have each bit set
        result << (nums.len() - 1)
    }
}
