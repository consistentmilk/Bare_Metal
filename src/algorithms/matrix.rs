pub struct Matrix;

#[derive(Debug)]
pub enum MatrixError {
    InvalidDotProductDimensions,
}

impl Matrix {
    pub fn dot(x: Vec<f64>, y: Vec<f64>) -> Result<f64, MatrixError> {
        if x.len() != y.len() {
            return Err(MatrixError::InvalidDotProductDimensions);
        }

        let mut res: f64 = 0.0;

        for i in 0..x.len() {
            res += x[i] * y[i];
        }

        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_algs_matrix_dot_1() {
        let x: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0];
        let y: Vec<f64> = vec![2.0, 2.0, 2.0, 2.0];
        let expected: f64 = 20.0;

        assert_eq!(Matrix::dot(x, y).ok(), Some(expected));
    }
}
