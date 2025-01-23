pub struct Solution;

impl Solution {
    pub fn minimum_chairs(s: String) -> i32 {
        let mut res: i32 = 0;
        let mut empty_chairs: i32 = 0;

        for ch in s.chars() {
            match ch {
                'E' => {
                    if empty_chairs > 0 {
                        empty_chairs -= 1;
                    } else {
                        res += 1;
                    }
                }

                'L' => {
                    empty_chairs += 1;
                }

                _ => panic!("Invalid test case"),
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3168_1() {
        let input: String = "EEEEEEE".to_string();

        assert_eq!(Solution::minimum_chairs(input), 7);
    }

    #[test]
    fn test_3168_2() {
        let input: String = "ELELEEL".to_string();

        assert_eq!(Solution::minimum_chairs(input), 2);
    }

    #[test]
    fn test_3168_3() {
        let input: String = "ELEELEELLL".to_string();

        assert_eq!(Solution::minimum_chairs(input), 3);
    }
}
