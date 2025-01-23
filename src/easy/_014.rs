pub struct Solution {}

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        strs.into_iter()
            .reduce(|acc: String, curr: String| {
                acc.chars()
                    .zip(curr.chars())
                    .take_while(|(a, c)| a == c)
                    .map(|(c, _)| c)
                    .collect()
            })
            .unwrap()
    }
}

pub fn vec_str_to_vec_string(input: &[&str]) -> Vec<String> {
    input
        .into_iter()
        .map(|str: &&str| str.to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_014_one() {
        let test_strs: Vec<String> = vec_str_to_vec_string(&["racecar", "race"]);
        let expected: String = "race".into();
        assert_eq!(Solution::longest_common_prefix(test_strs), expected);
    }

    #[test]
    fn test_014_two() {
        let test_strs: Vec<String> = vec_str_to_vec_string(&["flower", "flow", "fly"]);
        let expected: String = "fl".into();
        assert_eq!(Solution::longest_common_prefix(test_strs), expected);
    }
}
