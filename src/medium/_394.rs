pub struct Solution;

impl Solution {
    pub fn decode_string(s: String) -> String {
        let mut stack: Vec<(usize, String)> = Vec::new();
        let (mut n, mut res) = (0, String::new());

        for c in s.chars() {
            match c {
                '[' => {
                    stack.push((n, res.clone()));
                    n = 0;
                    res.clear();
                }

                ']' => {
                    if let Some(last) = stack.pop() {
                        res = last.1 + res.repeat(last.0).as_str();
                    }
                }

                '0'..='9' => n = n * 10 + (c as u8 - b'0') as usize,

                c => res.push(c),
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_394_1() {
        let test = "3[a2[c]]".to_string();
        let expected = "accaccacc".to_string();

        assert_eq!(Solution::decode_string(test), expected);
    }

    #[test]
    fn test_394_2() {
        let test = "3[a]2[bc]".to_string();
        let expected = "aaabcbc".to_string();

        assert_eq!(Solution::decode_string(test), expected);
    }
}
