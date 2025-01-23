pub struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = Vec::with_capacity(s.len());

        for ch in s.chars() {
            match ch {
                '[' => stack.push(']'),

                '{' => stack.push('}'),

                '(' => stack.push(')'),

                ']' | '}' | ')' => {
                    if Some(ch) != stack.pop() {
                        return false;
                    }
                }

                _ => continue,
            }
        }

        stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_020_1() {
        assert!(Solution::is_valid("[{()}]".to_string()));
    }

    #[test]
    fn test_020_2() {
        assert!(Solution::is_valid("[{(((())))}]".to_string()));
    }

    #[test]
    fn test_020_3() {
        assert!(!Solution::is_valid("((())".to_string()));
    }
}
