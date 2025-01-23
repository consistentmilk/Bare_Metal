#![allow(unused)]
pub struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let m = nums1.len();
        let n = nums2.len();

        if m > n {
            return Self::find_median_sorted_arrays(nums2, nums1);
        }

        let mut lp = 0;
        let mut rp = m;

        while lp <= rp {
            let pa = (lp + rp) / 2;
            let pb = (m + n + 1) / 2 - pa;

            let mla = if pa == 0 { i32::MIN } else { nums1[pa - 1] };
            let mra = if pa == m { i32::MAX } else { nums1[pa] };

            let mlb = if pb == 0 { i32::MIN } else { nums2[pb - 1] };
            let mrb = if pb == n { i32::MAX } else { nums2[pb] };

            if (mla <= mrb) && (mlb <= mra) {
                if (m + n) % 2 == 0 {
                    return f64::from(mla.max(mlb) + mra.min(mrb)) / 2.0;
                } else {
                    return f64::from(mla.max(mlb));
                }
            } else if mla > mrb {
                rp = pa - 1;
            } else {
                lp = pa + 1;
            }
        }

        0.0
    }

    pub fn find_median_sorted_arrays_alt(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let m: usize = nums1.len();
        let n: usize = nums2.len();

        if m > n {
            return Self::find_median_sorted_arrays(nums2, nums1);
        }

        let mut left: usize = 0;
        let mut right: usize = m;

        while left <= right {
            let partition_a: usize = (left + right) / 2;
            let partition_b: usize = (m + n + 1) / 2 - partition_a;

            let max_left_a: i32 = if partition_a == 0 {
                i32::MIN
            } else {
                nums1[partition_a - 1]
            };

            let min_right_a: i32 = if partition_a == m {
                i32::MAX
            } else {
                nums1[partition_a]
            };

            let max_left_b: i32 = if partition_b == 0 {
                i32::MIN
            } else {
                nums2[partition_b - 1]
            };

            let min_right_b: i32 = if partition_b == n {
                i32::MAX
            } else {
                nums2[partition_b]
            };

            if (max_left_a <= min_right_b) && (max_left_b <= min_right_a) {
                if (m + n) % 2 == 0 {
                    return f64::from(max_left_a.max(max_left_b) + min_right_a.min(min_right_b))
                        / 2.0;
                } else {
                    return f64::from(max_left_a.max(max_left_b));
                }
            } else if max_left_a > min_right_b {
                right = partition_a - 1;
            } else {
                left = partition_a + 1;
            }
        }

        0.0
    }

    pub fn naive_solution(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let n: usize = nums1.len() + nums2.len();
        let mut nums: Vec<i32> = Vec::with_capacity(n);

        nums.extend(nums1);
        nums.extend(nums2);

        nums.sort_unstable();

        if n % 2 == 0 {
            f64::from(nums[(n / 2) - 1] + nums[n / 2]) / 2.0
        } else {
            f64::from(nums[n / 2])
        }
    }

    pub fn get_min(nums1: &Vec<i32>, nums2: &Vec<i32>, p1: &mut usize, p2: &mut usize) -> i32 {
        if *p1 < nums1.len() && *p2 < nums2.len() {
            if nums1[*p1] < nums2[*p2] {
                let val = nums1[*p1];
                *p1 += 1;
                val
            } else {
                let val = nums2[*p2];
                *p2 += 1;
                val
            }
        } else if *p1 < nums1.len() {
            let val = nums1[*p1];
            *p1 += 1;
            val
        } else if *p2 < nums2.len() {
            let val = nums2[*p2];
            *p2 += 1;
            val
        } else {
            -1 // Shouldn't reach here if inputs are valid.
        }
    }

    pub fn find_median_sorted_arrays_alt_2(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut p1 = 0;
        let mut p2 = 0;
        let m = nums1.len();
        let n = nums2.len();
        let total_len = m + n;

        if total_len % 2 == 0 {
            for _ in 0..(total_len / 2 - 1) {
                Self::get_min(&nums1, &nums2, &mut p1, &mut p2);
            }

            let first = Self::get_min(&nums1, &nums2, &mut p1, &mut p2);
            let second = Self::get_min(&nums1, &nums2, &mut p1, &mut p2);

            (first + second) as f64 / 2.0
        } else {
            for _ in 0..(total_len / 2) {
                Self::get_min(&nums1, &nums2, &mut p1, &mut p2);
            }

            Self::get_min(&nums1, &nums2, &mut p1, &mut p2) as f64
        }
    }
}
