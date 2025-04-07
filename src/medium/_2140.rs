pub struct Solution;

impl Solution {
    pub fn most_points(questions: Vec<Vec<i32>>) -> i64 {
        let n: usize = questions.len();

        let mut dp: Vec<i64> = vec![0; n + 1];

        for i in (0..n).rev() {
            let (point, mut skip) = (questions[i][0] as i64, questions[i][1] as usize + i + 1);

            skip = std::cmp::min(n, skip);

            dp[i] = std::cmp::max(point + dp[skip], dp[i + 1]);
        }

        dp[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2140_1() {
        let questions: Vec<Vec<i32>> = vec![vec![3, 2], vec![4, 3], vec![4, 4], vec![2, 5]];
        let expected: i64 = 5;
        let result: i64 = Solution::most_points(questions);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_2140_2() {
        let questions: Vec<Vec<i32>> =
            vec![vec![1, 1], vec![2, 2], vec![3, 3], vec![4, 4], vec![5, 5]];
        let expected: i64 = 7;
        let result: i64 = Solution::most_points(questions);

        assert_eq!(result, expected);
    }
}
