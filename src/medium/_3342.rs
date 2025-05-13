/*
Intuition:
We perform a modified Dijkstra's algorithm on the grid.
Each cell has a required minimum wait time. Moving to a
neighbor requires waiting until the cell requirement
is met, then incurring a parity-based cost. Marking
visited cells in place avoids revisiting and reduces
memory overhead.

Algorithm:
1. Compute grid dimensions: rows and cols.
2. Initialize a min-heap of State structs and reserve
   rows*cols capacity.
3. Mark the start cell visited by setting grid[0][0] = -1.
4. Push State { time:0, x:0, y:0 } into the heap.
5. While the heap is not empty:
   a. Pop the State with the smallest time.
   b. For each of four directions:
      i. Compute neighbor coords (nx, ny).
     ii. Skip out-of-bounds or cells marked visited.
    iii. Compute arrival = max(time, grid[nx][ny]) + parity
         cost, where parity cost = ((x ^ y)&1) + 1.
     iv. If (nx,ny) is the destination, return arrival.
      v. Mark grid[nx][ny] = -1 and push State into heap.
6. unreachable!() if destination never reached.

Time Complexity:
Each cell is pushed and popped once. Heap ops cost
O(log(rows*cols)), so overall O(rows*cols*log(rows*cols)).

Space Complexity:
O(rows*cols) for the heap and input grid storage.
*/

// Import Ordering for custom comparison in State
use std::cmp::Ordering;
// Import BinaryHeap to implement a min-heap
use std::collections::BinaryHeap;

pub struct Solution;

#[derive(Eq, PartialEq)]
// Define State to hold the time and coordinates of a cell
struct State {
    // The current accumulated time to reach this cell
    time: i32,
    // The row index of this cell
    x: usize,
    // The column index of this cell
    y: usize,
}

// Implement Ord to allow State to be compared in the heap
impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        // Reverse compare to create a min-heap behavior
        other.time.cmp(&self.time)
    }
}

// Provide PartialOrd based on the Ord implementation
impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    // Compute minimal time to reach bottom-right corner
    pub fn min_time_to_reach(mut grid: Vec<Vec<i32>>) -> i32 {
        // Number of rows in the grid
        let rows: usize = grid.len();

        // Number of columns in the grid
        let cols: usize = grid[0].len();

        // Initialize a binary heap to store states by time
        let mut heap: BinaryHeap<State> = BinaryHeap::new();

        // Reserve capacity to avoid repeated reallocations
        heap.reserve(rows * cols);

        // Mark the start cell as visited
        grid[0][0] = -1;

        // Push the starting position with time=0 into the heap
        heap.push(State {
            time: 0,
            x: 0,
            y: 0,
        });

        // Movement directions: right, down, left, up
        const DIRS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

        // Continue processing until no states remain
        while let Some(State { time, x, y }) = heap.pop() {
            // Explore neighbors in each direction
            for &(dx, dy) in &DIRS {
                // Calculate neighbor's row index
                let nx_i: i32 = x as i32 + dx;

                // Calculate neighbor's column index
                let ny_i: i32 = y as i32 + dy;

                // Skip if neighbor is out of bounds
                if nx_i < 0 || nx_i >= rows as i32 || ny_i < 0 || ny_i >= cols as i32 {
                    continue;
                }

                // Convert to usize indices for grid access
                let nx: usize = nx_i as usize;
                let ny: usize = ny_i as usize;

                // Read the cell value to check visitation
                let cell: i32 = grid[nx][ny];

                // Skip if the cell has been visited
                if cell < 0 {
                    continue;
                }

                // Compute parity-based cost: 1 if even coord sum, 2 if odd
                let parity_cost: i32 = ((x ^ y) & 1) as i32 + 1;

                // Determine arrival time at the neighbor
                let arrival: i32 = time.max(cell) + parity_cost;

                // If neighbor is the destination, return arrival time
                if nx == rows - 1 && ny == cols - 1 {
                    return arrival;
                }

                // Mark the neighbor as visited before pushing
                grid[nx][ny] = -1;

                // Push the neighbor State onto the heap
                heap.push(State {
                    time: arrival,
                    x: nx,
                    y: ny,
                });
            }
        }

        // Panic if the destination is unreachable
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "entered unreachable code")]
    fn test_3342_single_cell_01() {
        // A 1×1 grid has no moves;
        // current implementation panics at unreachable!()
        let grid = vec![vec![0]];
        let _ = Solution::min_time_to_reach(grid);
    }

    #[test]
    fn test_3342_two_by_two_zero_02() {
        // Create a 2x2 grid of zeros.
        let grid = vec![vec![0, 0], vec![0, 0]];
        // Compute minimal time.
        let result = Solution::min_time_to_reach(grid);
        // Expected path cost: 3.
        assert_eq!(result, 3);
    }

    #[test]
    fn test_3342_wait_requirement_03() {
        // Create a 2x2 grid where one cell requires waiting.
        let grid = vec![vec![0, 5], vec![1, 1]];
        // Compute minimal time.
        let result = Solution::min_time_to_reach(grid);
        // Optimal path (down → right) yields cost 4.
        assert_eq!(result, 4);
    }

    #[test]
    fn test_3342_three_by_three_zero_04() {
        // Create a 3x3 grid of zeros.
        let grid = vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]];
        // Compute minimal time.
        let result = Solution::min_time_to_reach(grid);
        // Any shortest path takes 4 moves: cost = 6.
        assert_eq!(result, 6);
    }
}
