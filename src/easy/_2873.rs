pub struct Solution;

impl Solution {
    pub fn maximum_triplet_value(nums: Vec<i32>) -> i64 {
        let n: usize = nums.len();
        let mut res: i64 = 0;

        for i in 0..n {
            for j in i + 1..n {
                for k in j + 1..n {
                    res = res.max((nums[i] - nums[j]) as i64 * nums[k] as i64);
                }
            }
        }

        res
    }

    pub fn maximum_triplet_value_greedy(nums: Vec<i32>) -> i64 {
        let n: usize = nums.len();
        let mut res: i64 = 0;

        for k in 2..n {
            let mut max_prefix: i32 = nums[0];

            for j in 1..k {
                res = res.max((max_prefix - nums[j]) as i64 * nums[k] as i64);
                max_prefix = max_prefix.max(nums[j]);
            }
        }

        res
    }

    pub fn maximum_triplet_value_greedy_prefix_array(nums: Vec<i32>) -> i64 {
        let n: usize = nums.len();
        let mut left_max: Vec<i32> = vec![0; n];
        let mut right_max: Vec<i32> = vec![0; n];

        for i in 1..n {
            left_max[i] = left_max[i - 1].max(nums[i - 1]);
            right_max[n - i - 1] = right_max[n - i].max(nums[n - i]);
        }

        let mut res: i64 = 0;

        for j in 1..n - 1 {
            res = res.max((left_max[j] - nums[j]) as i64 * right_max[j] as i64);
        }

        res
    }

    pub fn maximum_triplet_value_optimized(nums: Vec<i32>) -> i64 {
        let mut res: i64 = 0;
        let mut imax: i64 = 0;
        let mut dmax: i64 = 0;

        for &num in nums.iter() {
            res = res.max(dmax * num as i64);
            dmax = dmax.max(imax - num as i64);
            imax = imax.max(num as i64);
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_all_implementations(nums: Vec<i32>) -> Vec<i64> {
        vec![
            Solution::maximum_triplet_value(nums.clone()),
            Solution::maximum_triplet_value_greedy(nums.clone()),
            Solution::maximum_triplet_value_greedy_prefix_array(nums.clone()),
            Solution::maximum_triplet_value_optimized(nums),
        ]
    }

    #[test]
    fn test_2873_1() {
        let nums = vec![1, 2, 3];
        let results = run_all_implementations(nums);
        assert!(
            results.iter().all(|&x| x == 0),
            "All implementations should return 0"
        );
    }

    #[test]
    fn test_2873_2() {
        let nums = vec![10, 5, 8, 3];
        let results = run_all_implementations(nums);
        assert!(
            results.iter().all(|&x| x == 40),
            "All implementations should return 40"
        );
    }

    #[test]
    fn test_2873_3() {
        let nums = vec![5, 2, 1, 4, 3];
        let results = run_all_implementations(nums);
        assert!(
            results.iter().all(|&x| x == 16),
            "All implementations should return 16"
        );
    }

    #[test]
    fn test_2873_4() {
        let nums = vec![100, 1, 100];
        let results = run_all_implementations(nums);
        assert!(
            results.iter().all(|&x| x == 9900),
            "All implementations should return 9900"
        );
    }

    #[test]
    fn test_2873_5() {
        let nums = vec![1, 1, 1];
        let results = run_all_implementations(nums);
        assert!(
            results.iter().all(|&x| x == 0),
            "All implementations should return 0"
        );
    }

    #[test]
    fn test_2873_6() {
        let nums = vec![5, 4, 3, 2, 1];
        let results = run_all_implementations(nums);
        assert!(
            results.iter().all(|&x| x == 4),
            "All implementations should return 4"
        );
    }

    #[test]
    fn test_2873_7() {
        let nums = vec![1, 2, 3, 4, 5];
        let results = run_all_implementations(nums);
        assert!(
            results.iter().all(|&x| x == 0),
            "All implementations should return 0"
        );
    }

    #[test]
    fn test_2873_8() {
        let nums = vec![10, 1, 10, 1, 10];
        let results = run_all_implementations(nums);
        assert!(
            results.iter().all(|&x| x == 90),
            "All implementations should return 90"
        );
    }

    #[test]
    fn test_2873_9() {
        let nums = vec![1000, 1, 1000];
        let results = run_all_implementations(nums);
        assert!(
            results.iter().all(|&x| x == 999000),
            "All implementations should return 999000"
        );
    }

    #[test]
    fn test_2873_10() {
        let nums = vec![3, 5, 2, 1, 4];
        let results = run_all_implementations(nums);
        assert!(
            results.iter().all(|&x| x == 16),
            "All implementations should return 16"
        );
    }

    #[test]
    fn test_2873_11() {
        let nums = vec![0, 0, 0];
        let results = run_all_implementations(nums);
        assert!(
            results.iter().all(|&x| x == 0),
            "All implementations should return 0"
        );
    }

    #[test]
    fn test_2873_12() {
        let nums = vec![1, 10, 2, 9, 3, 8];
        let results = run_all_implementations(nums);
        assert!(
            results.iter().all(|&x| x == 72),
            "All implementations should return 72"
        );
    }

    #[test]
    fn test_2873_13() {
        let nums = vec![100, 50, 75, 25];
        let results = run_all_implementations(nums);
        assert!(
            results.iter().all(|&x| x == 3750),
            "All implementations should return 3750"
        );
    }

    #[test]
    fn test_2873_14() {
        let nums = vec![4, 6, 2, 5, 3];
        let results = run_all_implementations(nums);
        assert!(
            results.iter().all(|&x| x == 20),
            "All implementations should return 20"
        );
    }

    #[test]
    fn test_2873_15() {
        let nums = vec![7, 7, 7, 7];
        let results = run_all_implementations(nums);
        assert!(
            results.iter().all(|&x| x == 0),
            "All implementations should return 0"
        );
    }

    #[test]
    fn test_2873_16() {
        let nums = vec![1, 2, 1, 2, 1];
        let results = run_all_implementations(nums);
        assert!(
            results.iter().all(|&x| x == 2),
            "All implementations should return 2"
        );
    }

    #[test]
    fn test_2873_17() {
        let nums = vec![10, 20, 10, 30, 20];
        let results = run_all_implementations(nums);
        assert!(
            results.iter().all(|&x| x == 300),
            "All implementations should return 300"
        );
    }

    #[test]
    fn test_2873_18() {
        let nums = vec![100, 200, 150, 50];
        let results = run_all_implementations(nums);
        assert!(
            results.iter().all(|&x| x == 2500),
            "All implementations should return 2500"
        );
    }

    #[test]
    fn test_2873_19() {
        let nums = vec![5, 10, 5, 15, 10];
        let results = run_all_implementations(nums);
        assert!(
            results.iter().all(|&x| x == 75),
            "All implementations should return 75"
        );
    }

    #[test]
    fn test_2873_20() {
        let nums = vec![1, 2, 3, 2, 1];
        let results = run_all_implementations(nums);
        assert!(
            results.iter().all(|&x| x == 1),
            "All implementations should return 1"
        );
    }
}
