pub struct Solution;

impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        let n: usize = nums.len();

        if n == 1 {
            return 0;
        }

        if nums[0] > nums[1] {
            return 0;
        }

        if nums[n - 1] > nums[n - 2] {
            return (n - 1) as i32;
        }

        let mut lp: usize = 1;
        let mut rp: usize = n - 2;

        while lp <= rp {
            let mid: usize = lp + (rp - lp) / 2;

            if (nums[mid] > nums[mid + 1]) && (nums[mid] > nums[mid - 1]) {
                return mid as i32;
            } else if nums[mid] < nums[mid + 1] {
                lp = mid + 1;
            } else {
                rp = mid - 1;
            }
        }

        -1
    }
}
