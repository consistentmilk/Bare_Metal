pub struct Solution {}

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        matrix.reverse();

        let n: usize = matrix.len();

        for i in 0..n {
            for j in i + 1..n {
                let a: i32 = matrix[i][j];
                let b: i32 = matrix[j][i];

                matrix[i][j] = b;
                matrix[j][i] = a;
            }
        }
    }
}
