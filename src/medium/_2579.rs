pub struct Solution;

impl Solution {
    pub fn colored_cells(n: i32) -> i64 {
        let mut total: i64 = 1;
        let mut increment: i64 = 4;

        for _ in 0..(n - 1) {
            total += increment;
            increment += 4;
        }

        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2579_1() {
        assert_eq!(Solution::colored_cells(1), 1);

        assert_eq!(Solution::colored_cells(2), 5);

        assert_eq!(Solution::colored_cells(3), 13);
    }
}
