pub struct Solution {}

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let m: usize = m as usize;
        let n: usize = n as usize;

        for i in m..(m + n) {
            nums1[i] = nums2[i - m];
        }

        nums1.sort_unstable();
    }
}
