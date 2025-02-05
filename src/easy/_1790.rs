pub struct Solution;

impl Solution {
    pub fn are_almost_equal(s1: String, s2: String) -> bool {
        let n: usize = s1.len();
        let s1_bytes: Vec<u8> = s1.into_bytes();
        let s2_bytes: Vec<u8> = s2.into_bytes();

        let mut first_diff_idx = 0;
        let mut second_diff_idx = 0;
        let mut diffs = 0;

        for i in 0..n {
            if s1_bytes[i] != s2_bytes[i] {
                diffs += 1;

                if diffs > 2 {
                    return false;
                } else if diffs == 1 {
                    first_diff_idx = i;
                } else {
                    second_diff_idx = i;
                }
            }
        }

        s1_bytes[first_diff_idx] == s2_bytes[second_diff_idx]
            && s1_bytes[second_diff_idx] == s2_bytes[first_diff_idx]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1790_1() {
        let s1: String = "bank".into();
        let s2: String = "kanb".into();

        assert!(Solution::are_almost_equal(s1, s2));
    }

    #[test]
    fn test_1790_2() {
        let s1: String = "attack".into();
        let s2: String = "defend".into();

        assert!(!Solution::are_almost_equal(s1, s2));
    }

    #[test]
    fn test_1790_3() {
        let s1: String = "kelb".into();
        let s2: String = "kelb".into();

        assert!(Solution::are_almost_equal(s1, s2));
    }
}
