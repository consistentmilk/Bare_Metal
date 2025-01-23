pub struct Solution {}

// Input: grid = [[0,1,0,0],[1,1,1,0],[0,1,0,0],[1,1,0,0]]
// Output: 16
impl Solution {
    pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        let mut islands: i32 = 0;
        let mut neighbors: i32 = 0;

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 1 {
                    islands += 1;

                    if i > 0 && grid[i - 1][j] == 1 {
                        neighbors += 1;
                    }

                    if j > 0 && grid[i][j - 1] == 1 {
                        neighbors += 1;
                    }
                }
            }
        }

        islands * 4 - neighbors * 2
    }
}
