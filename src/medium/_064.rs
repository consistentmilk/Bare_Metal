pub struct Solution;

impl Solution {
    pub fn min_path_sum(mut grid: Vec<Vec<i32>>) -> i32 {
        let rows: usize = grid.len();
        let cols: usize = grid[0].len();

        for i in 1..cols {
            grid[0][i] += grid[0][i - 1];
        }

        for i in 1..rows {
            grid[i][0] += grid[i - 1][0];
        }

        for i in 1..rows {
            for j in 1..cols {
                grid[i][j] += std::cmp::min(grid[i - 1][j], grid[i][j - 1])
            }
        }

        grid[rows - 1][cols - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_064_1() {
        let grid: Vec<Vec<i32>> = [[1, 3, 1], [1, 5, 1], [4, 2, 1]]
            .iter()
            .map(|v: &[i32; 3]| v.to_vec())
            .collect();
        let expected: i32 = 7;
        let res: i32 = Solution::min_path_sum(grid);

        assert_eq!(expected, res);
    }

    #[test]
    fn test_064_2() {
        let grid: Vec<Vec<i32>> = [[1, 2, 3], [4, 5, 6]]
            .iter()
            .map(|v: &[i32; 3]| v.to_vec())
            .collect();
        let expected: i32 = 12;
        let res: i32 = Solution::min_path_sum(grid);

        assert_eq!(expected, res);
    }
}
