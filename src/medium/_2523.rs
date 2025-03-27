pub struct Solution;

impl Solution {
    pub fn closest_primes(mut left: i32, right: i32) -> Vec<i32> {
        let mut step: usize = 2;

        if left & 1 == 0 && left > 2 {
            left += 1;
        }

        if left <= 2 {
            step = 1;
        }

        let mut res: Vec<i32> = vec![-1, -1];
        let mut last: i32 = 0;

        for n in (left..=right).step_by(step) {
            if Self::is_prime(n) {
                if last != 0 {
                    let prev_diff: i32 = res[1] - res[0];
                    let curr_diff: i32 = n - last;

                    if res[0] == -1 || curr_diff < prev_diff {
                        res[0] = last;
                        res[1] = n;
                    }
                }

                last = n;
            }
        }

        res
    }

    fn is_prime(n: i32) -> bool {
        if n < 2 {
            return false;
        }

        if n == 2 || n == 3 {
            return true;
        }

        if n & 1 == 0 {
            return false;
        }

        let bound: i32 = (n as f64).sqrt().ceil() as i32;

        for d in (3..=bound).step_by(2) {
            if n % d == 0 {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2523_1() {
        let left: i32 = 10;
        let right: i32 = 19;
        let expected: Vec<i32> = vec![11, 13];
        let res: Vec<i32> = Solution::closest_primes(left, right);

        assert_eq!(expected, res);
    }

    #[test]
    fn test_2523_2() {
        let left: i32 = 4;
        let right: i32 = 6;
        let expected: Vec<i32> = vec![-1, -1];
        let res: Vec<i32> = Solution::closest_primes(left, right);

        assert_eq!(expected, res);
    }
}
