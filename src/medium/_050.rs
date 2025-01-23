pub struct Solution {}

impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        if n == 0 {
            return 1.0;
        }

        let mut result: f64 = 1.0;
        let mut base: f64 = x;
        let mut exp: i64 = n as i64;

        if exp < 0 {
            base = 1.0 / base;
            exp = -exp;
        }

        while exp > 0 {
            if exp % 2 == 1 {
                result *= base;
            }

            base *= base;
            exp /= 2;
        }

        result
    }
}
