pub struct Solution;

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.is_empty() {
            return Vec::from([-1, -1]);
        }

        let n: usize = nums.len();

        if n == 1 && nums[0] == target {
            return Vec::from([0, 0]);
        }

        let mut lp: usize = 0;
        let mut rp: usize = n - 1;

        while lp < rp {
            let mid: usize = lp + (rp - lp) / 2;

            if nums[mid] < target {
                lp = mid + 1;
            } else {
                rp = mid;
            }
        }

        if nums[rp] != target {
            return Vec::from([-1, -1]);
        }

        lp = rp;
        loop {
            if lp == 0 || nums[lp] != target {
                break;
            }

            lp -= 1;
        }

        while rp + 1 < n && nums[rp + 1] == target {
            rp += 1;
        }

        if lp == 0 && nums[lp] == target {
            return Vec::from([lp as i32, rp as i32]);
        } else {
            return Vec::from([(lp + 1) as i32, rp as i32]);
        }
    }
}

pub struct SolutionAlt;

impl SolutionAlt {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.is_empty() {
            return vec![-1, -1];
        }

        let left: i32 = Self::find_left(&nums, target);
        if left == -1 {
            return vec![-1, -1];
        }

        let right: i32 = Self::find_right(&nums, target);

        vec![left, right]
    }

    fn find_left(nums: &[i32], target: i32) -> i32 {
        let mut left: usize = 0;
        let mut right: usize = nums.len();

        while left < right {
            let mid: usize = left + (right - left) / 2;
            if nums[mid] < target {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        if left == nums.len() || nums[left] != target {
            -1
        } else {
            left as i32
        }
    }

    fn find_right(nums: &[i32], target: i32) -> i32 {
        let mut left: usize = 0;
        let mut right: usize = nums.len();

        while left < right {
            let mid: usize = left + (right - left) / 2;
            if nums[mid] > target {
                right = mid;
            } else {
                left = mid + 1;
            }
        }

        if right == 0 || nums[right - 1] != target {
            -1
        } else {
            (right - 1) as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_034_1() {
        let nums = vec![5, 7, 7, 8, 8, 10];
        let target = 8;
        let expected = vec![3, 4];

        assert_eq!(expected, Solution::search_range(nums, target));
    }

    #[test]
    fn test_034_2() {
        let nums = vec![5, 7, 7, 8, 8, 10];
        let target = 6;
        let expected = vec![-1, -1];

        assert_eq!(expected, Solution::search_range(nums, target));
    }

    #[test]
    fn test_034_3() {
        let nums = vec![1, 4];
        let target = 4;
        let expected = vec![1, 1];

        assert_eq!(expected, Solution::search_range(nums, target));
    }

    #[test]
    fn test_034_4() {
        let nums = vec![1];
        let target = 1;
        let expected = vec![0, 0];

        assert_eq!(expected, Solution::search_range(nums, target));
    }

    #[test]
    fn test_034_5() {
        let nums = vec![2, 2];
        let target = 2;
        let expected = vec![0, 1];

        assert_eq!(expected, Solution::search_range(nums, target));
    }
}
