pub struct Solution {}

impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut stack: Vec<usize> = Vec::new();
        let n: usize = s.len();
        stack.push(0);

        let s: Vec<u8> = s.into_bytes();

        let mut ans: usize = 0;

        for i in 1..=n {
            if s[i - 1] == b'(' {
                stack.push(i);
            } else {
                stack.pop();

                if stack.len() == 0 {
                    stack.push(i);
                } else {
                    ans = usize::max(ans, i - stack[stack.len() - 1]);
                }
            }
        }

        ans as i32
    }
}
