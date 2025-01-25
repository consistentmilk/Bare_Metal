pub struct Solution;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();

        fn backtrack(
            current: String,
            open_count: i32,
            close_count: i32,
            n: i32,
            result: &mut Vec<String>,
        ) {
            if open_count == n && close_count == n {
                result.push(current.clone());
                return;
            }

            if open_count < n {
                backtrack(
                    format!("{}(", current),
                    open_count + 1,
                    close_count,
                    n,
                    result,
                );
            }

            if close_count < open_count {
                backtrack(
                    format!("{})", current),
                    open_count,
                    close_count + 1,
                    n,
                    result,
                );
            }
        }

        backtrack(String::new(), 0, 0, n, &mut result);

        return result;
    }
}
