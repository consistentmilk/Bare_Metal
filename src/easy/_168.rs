pub struct Solution;

impl Solution {
    pub fn convert_to_title(column_number: i32) -> String {
        let mut n: i32 = column_number;
        let mut ans: String = "".to_string();

        while n > 0 {
            n -= 1;

            ans.push(((n % 26) as u8 + 'A' as u8) as char);

            n = n / 26;
        }

        ans.chars().rev().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_168_one() {
        let column_num: i32 = 701;
        let expected: String = "ZY".to_string();

        assert_eq!(Solution::convert_to_title(column_num), expected);
    }

    #[test]
    fn test_168_two() {
        let column_num: i32 = 2002;
        let expected: String = "BXZ".to_string();

        assert_eq!(Solution::convert_to_title(column_num), expected);
    }
}
