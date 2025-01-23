pub struct Solution;

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let n: usize = nums.len();
        let mut result: Vec<Vec<i32>> = Vec::with_capacity(1 << n);

        for i in 0..(1 << n) {
            let mut subset: Vec<i32> = Vec::with_capacity(n);

            for j in 0..n {
                if (i & (1 << j)) != 0 {
                    subset.push(nums[j]);
                }
            }

            result.push(subset);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_078_1() {
        let input: Vec<i32> = vec![1, 2, 3];
        let expected: Vec<Vec<i32>> = vec![
            vec![],
            vec![1],
            vec![2],
            vec![1, 2],
            vec![3],
            vec![1, 3],
            vec![2, 3],
            vec![1, 2, 3],
        ];

        assert_eq!(Solution::subsets(input), expected);
    }

    #[test]
    fn test_078_2() {
        let input: Vec<i32> = vec![0];
        let expected: Vec<Vec<i32>> = vec![vec![], vec![0]];

        assert_eq!(Solution::subsets(input), expected);
    }
}
