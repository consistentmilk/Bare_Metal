use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn length_of_longest_substring_opt(s: String) -> i32 {
        let mut chr_tbl: [i32; 128] = [-1; 128];
        let mut lp: i32 = 0;
        let mut max_len: i32 = 0;

        for (rp, ch) in s.as_bytes().iter().enumerate() {
            if chr_tbl[*ch as usize] >= lp {
                lp = chr_tbl[*ch as usize] + 1;
            } else {
                let curr_len: i32 = rp as i32 - lp + 1;

                if curr_len > max_len {
                    max_len = curr_len;
                }
            }

            chr_tbl[*ch as usize] = rp as i32;
        }

        max_len
    }

    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut h_tbl: HashMap<char, usize> = HashMap::new();
        let mut lp: usize = 0;
        let mut maxlen: usize = 0;

        for (rp, ch) in s.char_indices() {
            if let Some(prev_lp) = h_tbl.insert(ch, rp) {
                lp = std::cmp::max(lp, prev_lp + 1);
            }

            maxlen = std::cmp::max(maxlen, rp - lp + 1);
        }

        maxlen as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_003_1() {
        assert_eq!(
            Solution::length_of_longest_substring("abcabcbb".to_string()),
            3
        );
    }

    #[test]
    fn test_003_2() {
        assert_eq!(
            Solution::length_of_longest_substring("bbbbb".to_string()),
            1
        );
    }

    #[test]
    fn test_003_3() {
        assert_eq!(
            Solution::length_of_longest_substring("pwwkew".to_string()),
            3
        );
    }

    #[test]
    fn test_003_4() {
        assert_eq!(Solution::length_of_longest_substring("".to_string()), 0);
    }
}
