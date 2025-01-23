pub struct Solution;

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut direction: Direction = Direction::Right;
        let mut round: usize = 0;
        let mut row: usize = 0;
        let mut col: usize = 0;
        let m: usize = matrix.len();
        let n: usize = matrix[0].len();

        let mut flatten_matrix: Vec<i32> = vec![];

        for _ in 0..m * n {
            flatten_matrix.push(matrix[row][col]);

            match direction {
                Direction::Up => {
                    if row == round + 1 {
                        col += 1;
                        direction = Direction::Right;
                        round += 1;
                    } else {
                        row -= 1;
                    }
                }

                Direction::Down => {
                    if row + 1 == m - round {
                        col -= 1;
                        direction = Direction::Left;
                    } else {
                        row += 1;
                    }
                }

                Direction::Right => {
                    if col + 1 == n - round {
                        row += 1;
                        direction = Direction::Down;
                    } else {
                        col += 1;
                    }
                }

                Direction::Left => {
                    if col == round {
                        row -= 1;
                        direction = Direction::Up;
                    } else {
                        col -= 1;
                    }
                }
            }
        }

        flatten_matrix
    }
}
