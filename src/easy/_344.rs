pub struct Solution;

impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        let mut lp: usize = 0;
        let mut rp: usize = s.len() - 1;

        while lp < rp {
            let tmp: char = s[lp];

            s[lp] = s[rp];
            s[rp] = tmp;

            lp += 1;
            rp -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_344_1() {
        let mut s: Vec<char> = vec!['h', 'e', 'l', 'l', 'o'];
        let expected: Vec<char> = vec!['o', 'l', 'l', 'e', 'h'];

        Solution::reverse_string(&mut s);

        assert_eq!(s, expected);
    }
}
