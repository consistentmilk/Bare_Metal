pub struct Solution;

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let mut i: usize = 0;
        let mut j: usize = 0;
        let s_chars: &[u8] = s.as_bytes();
        let t_chars: &[u8] = t.as_bytes();

        while i < s.len() && j < t.len() {
            if s_chars[i] == t_chars[j] {
                i += 1;
            }

            j += 1;
        }

        i == s.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_392_1() {
        assert!(Solution::is_subsequence("abc".into(), "ahbgdc".into()));
    }

    #[test]
    fn test_392_2() {
        assert!(!Solution::is_subsequence("axc".into(), "ahbgdc".into()))
    }
}
