use std::iter::Peekable;
use std::str::Chars;

pub struct Solution;

impl Solution {
    pub fn compressed_string(word: String) -> String {
        let mut res: String = String::with_capacity(word.len() * 2);
        let mut it: Peekable<Chars> = word.chars().peekable();
        let mut curr_count: u32 = 1;

        while let Some(ch) = it.next() {
            if let Some(&next_ch) = it.peek() {
                if ch == next_ch && curr_count < 9 {
                    curr_count += 1;
                } else {
                    res.push(char::from_digit(curr_count, 10).unwrap());
                    res.push(ch);

                    curr_count = 1;
                }
            } else {
                res.push(char::from_digit(curr_count, 10).unwrap());
                res.push(ch);
            }
        }

        res
    }

    pub fn compressed_string_alt(word: String) -> String {
        let n: usize = word.len();
        let mut ans: String = String::with_capacity(n * 2);
        let mut count: u32 = 0;
        let mut curr_ch: char = 'a';

        for ch in word.chars() {
            if count == 0 {
                count = 1;
                curr_ch = ch;
            } else if ch != curr_ch || count == 9 {
                ans.push(char::from_digit(count, 10).unwrap());
                ans.push(curr_ch);

                count = 1;
                curr_ch = ch;
            } else if ch == curr_ch {
                count += 1;
            }
        }

        if count > 0 {
            ans.push(char::from_digit(count, 10).unwrap());
            ans.push(curr_ch);
        }

        ans.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3163_1() {
        let word: String = "abcde".into();
        let expected: String = "1a1b1c1d1e".into();

        assert_eq!(Solution::compressed_string(word), expected);
    }

    #[test]
    fn test_3163_2() {
        let word: String = "aaaaaaaaaaaaaabb".into();
        let expected: String = "9a5a2b".into();

        assert_eq!(Solution::compressed_string(word), expected);
    }
}
