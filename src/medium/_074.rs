use std::cmp::Ordering;

pub struct Solution;

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let rows: i32 = matrix.len() as i32;
        let cols: i32 = matrix[0].len() as i32;

        let mut start: i32 = 0;
        let mut end: i32 = rows * cols - 1;

        while start <= end {
            let mid: i32 = start + (end - start) / 2;
            let mid_value: i32 = matrix[(mid / cols) as usize][(mid % cols) as usize];

            match mid_value.cmp(&target) {
                Ordering::Equal => {
                    return true;
                }

                Ordering::Less => {
                    start = mid + 1;
                }

                Ordering::Greater => {
                    end = mid - 1;
                }
            }
        }

        false
    }
}
