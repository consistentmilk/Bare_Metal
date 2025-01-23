use std::ops::RangeInclusive;

pub struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let m: usize = s.len();
        let bytes: &[u8] = s.as_bytes();
        let mut lp: usize = 0;
        let mut rp: usize = 0;
        let mut max_len: usize = 1;

        loop {
            let (start, length, consecutive_letters) = Self::palindrome_check(bytes, rp);

            if length > max_len {
                max_len = length;
                lp = start;
            }

            rp += consecutive_letters;

            if rp >= m {
                break;
            }
        }

        s[lp..(lp + max_len)].to_string()
    }

    fn palindrome_check(s: &[u8], start_idx: usize) -> (usize, usize, usize) {
        let m: usize = s.len();
        let ch: u8 = s[start_idx];
        let mut j: usize = 1;

        let consecutive_letters: usize = loop {
            if j + start_idx >= m || s[start_idx + j] != ch {
                break j;
            }

            j += 1;
        };

        let mut r_idx: usize;
        let mut j: usize = 1;
        let (l_idx, len) = loop {
            r_idx = start_idx + consecutive_letters - 1 + j;

            if j > start_idx || r_idx >= m || s[start_idx - j] != s[r_idx] {
                let l_idx: usize = start_idx + 1 - j;

                break (l_idx, r_idx - l_idx);
            }

            j += 1;
        };

        (l_idx, len, consecutive_letters)
    }
}

pub struct SolutionAltOne;

impl SolutionAltOne {
    #[must_use]
    pub fn longest_palindrome(s: String) -> String {
        let s: &str = s.as_str();

        (0..s.len())
            .fold("", |current_longest: &str, idx: usize| {
                current_longest
                    .longest(s.longest_palindrome_around(idx..=idx))
                    .longest(s.longest_palindrome_around(idx..=idx + 1))
            })
            .into()
    }
}

trait LongestPalindrome {
    type Idx;

    fn longest_palindrome_around(&self, center: RangeInclusive<Self::Idx>) -> &Self;

    fn longest<'a>(&'a self, other: &'a Self) -> &'a Self;
}

impl LongestPalindrome for str {
    type Idx = usize;

    fn longest_palindrome_around(&self, center: RangeInclusive<Self::Idx>) -> &Self {
        let (mut start, mut end) = center.into_inner();
        let chars: &[u8] = self.as_bytes();

        loop {
            if chars.get(start) != chars.get(end) {
                return &self[start + 1..end];
            }

            if let (Some(new_start), Some(new_end)) = (start.checked_sub(1), end.checked_add(1)) {
                start = new_start;
                end = new_end;
            } else {
                return &self[start..=end];
            }
        }
    }

    fn longest<'a>(&'a self, other: &'a Self) -> &'a Self {
        if self.len() > other.len() {
            self
        } else {
            other
        }
    }
}

pub struct SolutionAltTwo;

impl SolutionAltTwo {
    #[must_use]
    pub fn longest_palindrome(s: String) -> String {
        let s: Palindrome = Palindrome(s.as_str());

        (0..s.len())
            .fold(Palindrome(""), |curr_longest: Palindrome, idx: usize| {
                curr_longest
                    .max(s.expand_around(idx..=idx))
                    .max(s.expand_around(idx..=idx + 1))
            })
            .into()
    }
}

pub struct Palindrome<'a>(&'a str);

impl<'a> Palindrome<'a> {
    #[must_use]
    pub fn expand_around(&'a self, center: RangeInclusive<usize>) -> Self {
        let (mut lp, mut rp) = center.into_inner();
        let chars: &[u8] = self.0.as_bytes();

        loop {
            if chars.get(lp) != chars.get(rp) {
                return Self(&self.0[lp + 1..rp]);
            }

            if let (Some(new_lp), Some(new_rp)) = (lp.checked_sub(1), rp.checked_add(1)) {
                lp = new_lp;
                rp = new_rp;
            } else {
                return Self(&self.0[lp + 1..=rp]);
            }
        }
    }

    #[must_use]
    #[inline]
    pub const fn len(&self) -> usize {
        self.0.len()
    }
}

impl<'a> Ord for Palindrome<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.len().cmp(&other.len())
    }
}

impl<'a> PartialOrd for Palindrome<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.len().partial_cmp(&other.len())
    }
}

impl<'a> Eq for Palindrome<'a> {}

impl<'a> PartialEq for Palindrome<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.len().eq(&other.len())
    }
}

impl<'a> From<Palindrome<'a>> for String {
    fn from(value: Palindrome<'a>) -> Self {
        value.0.into()
    }
}

pub struct SolutionAltThree;

impl SolutionAltThree {
    pub fn longest_palindrome(s: String) -> String {
        if s == s.chars().rev().collect::<String>() {
            return s;
        }

        let mut start = 0;
        let mut end = 0;

        for i in 0..s.len() {
            // Odd-length palindrome (center at i)
            let (l1, r1) = Self::expand(&s, i, i);

            // Even-length palindrome (center between i and i + 1)
            let (l2, r2) = Self::expand(&s, i, i + 1);

            // Update the longest palindrome range
            if r1 - l1 > end - start {
                start = l1;
                end = r1;
            }
            if r2 - l2 > end - start {
                start = l2;
                end = r2;
            }
        }

        s[start..end].to_string()
    }

    #[allow(unused_comparisons)]
    fn expand(s: &str, mut lp: usize, mut rp: usize) -> (usize, usize) {
        let chars: &[u8] = s.as_bytes();

        while lp >= 0 && rp < chars.len() && chars[lp] == chars[rp] {
            if lp == 0 {
                return (lp, rp + 1);
            }

            lp -= 1;
            rp += 1;
        }

        (lp + 1, rp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_005_1() {
        let test = "babad".to_string();
        let expected = "bab".to_string();
        let expected_alt = "aba".to_string();
        let res = SolutionAltTwo::longest_palindrome(test);

        assert!(expected == res || expected_alt == res);
    }

    #[test]
    fn test_005_2() {
        let test = "cbbd".to_string();
        let expected = "bb".to_string();
        let res = SolutionAltTwo::longest_palindrome(test);

        assert_eq!(expected, res);
    }
}
