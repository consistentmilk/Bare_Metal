pub struct Problem100286 {}

// You are given a 2D matrix grid of size 3 x 3 consisting only of characters 'B' and 'W'. Character 'W' represents the white color, and character 'B' represents the black color.

// Your task is to change the color of at most one cell so that the matrix has a 2 x 2 square where all cells are of the same color.
// Return true if it is possible to create a 2 x 2 square of the same color, otherwise, return false.

// Example 1
// Input: grid = [["B","W","B"],["B","W","W"],["B","W","B"]]
// Output: true
// Explanation:
// B W B
// B W W
// B W B
// It can be done by changing the color of the grid[0][2].

// Example 2
// Input: grid = [["B","W","B"],["W","B","W"],["B","W","B"]]
// Output: false
// Explanation:
// B W B
// W B W
// B W B
// It cannot be done by changing at most one cell.

// Example 3
// Input: grid = [["B","W","B"],["B","W","W"],["B","W","W"]]
// Output: true
// Explanation:
// B W B
// B W W
// B W W
// The grid already contains a 2 x 2 square of the same color.

// Constraints:
// grid.length == 3
// grid[i].length == 3
// grid[i][j] is either 'W' or 'B'.
impl Problem100286 {
    pub fn can_make_square(grid: Vec<Vec<char>>) -> bool {
        for i in 0..2 {
            for j in 0..2 {
                let mut black_sqaures: i32 = 0;
                let mut white_squares: i32 = 0;

                for ii in 0..2 {
                    for jj in 0..2 {
                        black_sqaures += (grid[i + ii][j + jj] == 'B') as i32;
                        white_squares += (grid[i + ii][j + jj] == 'W') as i32;
                    }
                }

                if (black_sqaures >= 3) || (white_squares >= 3) {
                    return true;
                }
            }
        }

        false
    }
}

pub struct Problem3128 {}
// You are given a 2D boolean matrix grid.
// Return an integer that is the number of right triangles that can be made with the 3 elements of grid such that all of them have a value of 1.

// Note:
// A collection of 3 elements of grid is a right triangle if one of its elements is in the same row with another element and in the same column with the third element. The 3 elements do not have to be next to each other.

// Example 1
// Input: grid = [[0,1,0],[0,1,1],[0,1,0]]
// Output: 2

// Example 2
// Input: grid = [[1,0,0,0],[0,1,0,1],[1,0,0,0]]
// Output: 0

// Example 3
// Input: grid = [[1,0,1],[1,0,0],[1,0,0]]
// Output: 2

// Constraints:
// 1 <= grid.length <= 1000
// 1 <= grid[i].length <= 1000
// 0 <= grid[i][j] <= 1

impl Problem3128 {
    pub fn number_of_right_triangles(grid: Vec<Vec<i32>>) -> i64 {
        let m = grid.len();
        let n = grid[0].len();

        let mut rows = vec![0; m];
        let mut cols = vec![0; n];

        for i in 0..m {
            for j in 0..n {
                rows[i] += (grid[i][j] == 1) as i32;
                cols[j] += (grid[i][j] == 1) as i32;
            }
        }

        let mut ans: i64 = 0;

        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 0 {
                    continue;
                }

                ans += (1 * (rows[i] - 1) * (cols[j] - 1)) as i64;
            }
        }

        ans
    }
}

pub struct Problem3129 {}
// You are given 3 positive integers zero, one, and limit.

// A binary array arr is called stable if:
// The number of occurrences of 0 in arr is exactly zero.
// The number of occurrences of 1 in arr is exactly one.
// Each subarray of arr with a size greater than limit must contain both 0 and 1.

// Return the total number of stable binary arrays.
// Since the answer may be very large, return it modulo 10^9 + 7.

// Example 1
// Input: zero = 1, one = 1, limit = 2
// Output: 2
// Explanation:
// The two possible stable binary arrays are [1,0] and [0,1], as both arrays have a single 0 and a single 1, and no subarray has a length greater than 2.

// Example 2
// Input: zero = 1, one = 2, limit = 1
// Output: 1
// Explanation:
// The only possible stable binary array is [1,0,1].
// Note that the binary arrays [1,1,0] and [0,1,1] have subarrays of length 2 with identical elements, hence, they are not stable.

// Example 3
// Input: zero = 3, one = 3, limit = 2
// Output: 14
// Explanation:
// All the possible stable binary arrays are [0,0,1,0,1,1], [0,0,1,1,0,1], [0,1,0,0,1,1], [0,1,0,1,0,1], [0,1,0,1,1,0], [0,1,1,0,0,1], [0,1,1,0,1,0], [1,0,0,1,0,1], [1,0,0,1,1,0], [1,0,1,0,0,1], [1,0,1,0,1,0], [1,0,1,1,0,0], [1,1,0,0,1,0], and [1,1,0,1,0,0].

// Constraints:
// 1 <= zero, one, limit <= 200

impl Problem3129 {
    pub fn number_of_stable_arrays(zero: i32, one: i32, limit: i32) -> i32 {
        let n = (zero + one) as usize;
        let limit = limit as usize;
        let zero = zero as usize;
        const MOD: i32 = 1e9 as i32 + 7;

        let mut dp0 = vec![vec![0; n + 1]; n + 1];

        let mut dp1 = vec![vec![0; n + 1]; n + 1];

        dp0[0][0] = 1;
        dp1[0][0] = 1;
        dp0[1][1] = 1;
        dp1[1][0] = 1;

        for i in 2..=n as usize {
            for j in 0..=i as usize {
                let mut x = 0;
                let mut y = 0;

                if j >= 1 {
                    x = (x + dp0[i - 1][j - 1]) % MOD;
                }

                if j >= 1 {
                    x = (x + dp1[i - 1][j - 1]) % MOD;
                }

                if j >= limit + 1 {
                    x = (x - dp1[i - limit - 1][j - limit - 1]) % MOD;
                }

                if x < 0 {
                    x += MOD;
                }

                dp0[i][j] = x;

                y = (y + dp0[i - 1][j]) % MOD;
                y = (y + dp1[i - 1][j]) % MOD;

                if i >= limit + 1 {
                    y = (y - dp0[i - limit - 1][j]) % MOD;
                }

                if y < 0 {
                    y += MOD;
                }

                dp1[i][j] = y;
            }
        }

        let mut ans = (dp0[n][zero] + dp1[n][zero]) % MOD;

        if ans < 0 {
            ans += MOD
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bw_129_1() {
        let test_arr: Vec<Vec<char>> = vec![
            vec!['B', 'W', 'B'],
            vec!['B', 'W', 'W'],
            vec!['B', 'W', 'B'],
        ];
        let expected: bool = true;

        assert_eq!(Problem100286::can_make_square(test_arr), expected);
    }
}
