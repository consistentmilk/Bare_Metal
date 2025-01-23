pub struct Solution;

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut lp: usize = 0;

        for rp in 0..nums.len() {
            if nums[rp] != val {
                nums.swap(lp, rp);
                lp += 1;
            }
        }

        lp as i32
    }

    pub fn remove_element_alt(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut i: usize = 0;

        loop {
            let n: usize = nums.len();
            if i >= n {
                break;
            } else {
                if nums[i] == val {
                    nums.swap_remove(i);
                } else {
                    i += 1;
                }
            }
        }

        nums.len() as i32
    }
}
