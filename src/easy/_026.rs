pub struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut prev: usize = 0;

        for i in 1..nums.len() {
            if nums[prev] != nums[i] {
                prev += 1;
                nums.swap(prev, i);
            }
        }

        (prev + 1) as i32
    }
}
