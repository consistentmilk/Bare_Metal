pub struct Solution {}

impl Solution {
    pub fn permute(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let n: usize = nums.len();
        let mut result = Vec::new();

        fn backtrack(nums: &mut Vec<i32>, lp: usize, rp: usize, result: &mut Vec<Vec<i32>>) {
            if lp == rp {
                result.push(nums.clone());
                return;
            }

            for i in lp..=rp {
                // Swap elements
                nums.swap(lp, i);

                // Recursive call
                backtrack(nums, lp + 1, rp, result);

                // Backtrack by swapping back
                nums.swap(lp, i);
            }
        }

        backtrack(&mut nums, 0, n - 1, &mut result);

        result
    }

    /// Heap's algorithm for generating permutations
    /// Produced permutations do not follow the same order as
    /// the backtracking based solution.
    pub fn permute_alt(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = Vec::with_capacity(720);
        res.push(nums.clone());

        let n: usize = nums.len();
        let mut c: Vec<usize> = vec![0; n];
        let mut i: usize = 1;

        while i < n {
            if c[i] < i {
                if i % 2 == 0 {
                    nums.swap(0, i);
                } else {
                    nums.swap(c[i], i);
                }

                res.push(nums.clone());

                c[i] += 1;
                i = 1;
            } else {
                c[i] = 0;
                i += 1;
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_46_1() {
        let nums: Vec<i32> = vec![1, 2, 3];
        let mut expected: Vec<Vec<i32>> = [
            [1, 2, 3],
            [1, 3, 2],
            [2, 1, 3],
            [2, 3, 1],
            [3, 2, 1],
            [3, 1, 2],
        ]
        .iter()
        .map(|v: &[i32; 3]| v.to_vec())
        .collect::<Vec<Vec<i32>>>();
        expected.sort();

        let mut res: Vec<Vec<i32>> = Solution::permute(nums);
        res.sort();

        assert_eq!(expected, res);
    }

    #[test]
    fn test_46_2() {
        let nums: Vec<i32> = vec![0, 1];
        let mut expected: Vec<Vec<i32>> = vec![vec![0, 1], vec![1, 0]];
        expected.sort();

        let mut res: Vec<Vec<i32>> = Solution::permute(nums);
        res.sort();

        assert_eq!(expected, res);
    }

    #[test]
    fn test_46_3() {
        let nums: Vec<i32> = vec![1];
        let mut expected: Vec<Vec<i32>> = vec![vec![1]];
        expected.sort();

        let mut res: Vec<Vec<i32>> = Solution::permute(nums);
        res.sort();

        assert_eq!(expected, res);
    }

    #[test]
    fn test_46_4() {
        let nums: Vec<i32> = vec![1, 2, 3];
        let mut expected: Vec<Vec<i32>> = [
            [1, 2, 3],
            [1, 3, 2],
            [2, 1, 3],
            [2, 3, 1],
            [3, 2, 1],
            [3, 1, 2],
        ]
        .iter()
        .map(|v: &[i32; 3]| v.to_vec())
        .collect::<Vec<Vec<i32>>>();
        expected.sort();

        let mut res: Vec<Vec<i32>> = Solution::permute_alt(nums);
        res.sort();

        assert_eq!(expected, res);
    }

    #[test]
    fn test_46_5() {
        let nums: Vec<i32> = vec![0, 1];
        let mut expected: Vec<Vec<i32>> = vec![vec![0, 1], vec![1, 0]];
        expected.sort();

        let mut res: Vec<Vec<i32>> = Solution::permute_alt(nums);
        res.sort();

        assert_eq!(expected, res);
    }

    #[test]
    fn test_46_6() {
        let nums: Vec<i32> = vec![1];
        let mut expected: Vec<Vec<i32>> = vec![vec![1]];
        expected.sort();

        let mut res: Vec<Vec<i32>> = Solution::permute_alt(nums);
        res.sort();

        assert_eq!(expected, res);
    }
}
