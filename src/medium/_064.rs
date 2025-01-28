pub struct Solution;

impl Solution {
    /// Calculate the minimum sum path in a 2D grid.
    ///
    /// This function takes a 2D vector of integers `grid`, where each
    /// integer represents a cell in the grid. The function uses dynamic
    /// programming to find the minimum sum path from the top-left to the
    /// bottom-right of the grid.
    ///
    /// # Arguments
    ///
    /// * `grid` - A 2D vector of integers representing the grid.
    ///
    /// # Returns
    ///
    /// * The minimum sum of the path from the top-left to the
    ///   bottom-right of the grid.
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        // Get the dimensions of the grid.
        let m: usize = grid.len();
        let n: usize = grid[0].len();

        // Create a 2D vector to store the sum of paths.
        let mut dp: Vec<Vec<i32>> = vec![vec![0; n]; m];

        // Initialize the first cell of dp with the value of the first cell in the grid.
        dp[0][0] = grid[0][0];

        // Initialize the first row of dp with the sum of the values in the first row of the grid.
        for i in 1..m {
            dp[i][0] = dp[i - 1][0] + grid[i][0];
        }

        // Initialize the first column of dp with the sum of the values in the first column of the grid.
        for j in 1..n {
            dp[0][j] = dp[0][j - 1] + grid[0][j];
        }

        // Iterate over each cell in the grid, calculate the sum of paths using dynamic programming.
        for i in 1..m {
            for j in 1..n {
                // Calculate the sum of paths using the minimum value of the paths from the cell above and the cell to the left.
                dp[i][j] = grid[i][j] + i32::min(dp[i - 1][j], dp[i][j - 1]);
            }
        }

        // Return the sum of paths from the bottom-right cell of the grid.
        dp[m - 1][n - 1]
    }

    pub fn min_path_sum_alt(grid: Vec<Vec<i32>>) -> i32 {
        let rows: usize = grid.len();
        let cols: usize = grid[0].len();

        let mut dp: Vec<Vec<i32>> = vec![vec![0; cols]; rows];

        dp[0][0] = grid[0][0];

        for i in 1..rows {
            dp[i][0] = dp[i - 1][0] + grid[i][0];
        }

        for j in 1..cols {
            dp[0][j] = dp[0][j - 1] + grid[0][j];
        }

        for i in 1..rows {
            for j in 1..cols {
                dp[i][j] = i32::min(dp[i - 1][j], dp[i][j - 1]) + grid[i][j];
            }
        }

        dp[rows - 1][cols - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_path_sum() {
        let grid = vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]];

        assert_eq!(Solution::min_path_sum(grid), 7);
    }
}
