pub struct Solution {}

use std::collections::HashMap;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let roman: HashMap<char, i32> = HashMap::from([
            ('I', 1),
            ('V', 5),
            ('X', 10),
            ('L', 50),
            ('C', 100),
            ('D', 500),
            ('M', 1000),
        ]);

        let mut res: i32 = 0;
        let mut prev_val: i32 = 0;

        for ch in s.chars() {
            match roman.get(&ch) {
                Some(&val) => {
                    if val > prev_val {
                        res = res + val - (2 * prev_val);
                    } else {
                        res = res + val;
                    }

                    prev_val = val;
                }

                None => {}
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_013_one() {
        let test_str: String = "IV".into();
        let expected: i32 = 4;
        assert_eq!(Solution::roman_to_int(test_str), expected);
    }

    #[test]
    fn test_013_two() {
        let test_str: String = "MCDXLIV".into();
        let expected: i32 = 1444;
        assert_eq!(Solution::roman_to_int(test_str), expected);
    }

    #[test]
    fn test_013_three() {
        let test_str: String = "DCLXVI".into();
        let expected: i32 = 666;
        assert_eq!(Solution::roman_to_int(test_str), expected);
    }
}
