pub struct Solution;

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let mut dp: Vec<i32> = vec![1; n as usize];

        for _ in 1..m as usize {
            for j in 1..n as usize {
                dp[j] += dp[j - 1];
            }
        }

        dp[n as usize - 1]
    }

    pub fn unique_paths_alt(m: i32, n: i32) -> i32 {
        let m = m as usize;
        let n = n as usize;
        let mut dp: Vec<Vec<i32>> = vec![vec![0; n]; m];

        for i in 0..m {
            for j in 0..n {
                if i == 0 || j == 0 {
                    dp[i][j] = 1;
                } else {
                    dp[i][j] = dp[i - 1][j] + dp[i][j - 1];
                }
            }
        }

        dp[m - 1][n - 1]
    }
}
