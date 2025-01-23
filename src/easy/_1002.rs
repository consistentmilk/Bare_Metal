pub struct Solution;

impl Solution {
    pub fn common_chars(words: Vec<String>) -> Vec<String> {
        let mut freq_tbl: [u8; 26] = [u8::MAX; 26];

        for word in &words {
            let mut word_freq: [u8; 26] = [0; 26];

            for byte in word.as_bytes() {
                word_freq[(byte - b'a') as usize] += 1;
            }

            for i in 0..26 {
                freq_tbl[i] = freq_tbl[i].min(word_freq[i]);
            }
        }

        let mut res: Vec<String> = Vec::with_capacity(words[0].len() * words.len());
        for (base_idx, cnt) in freq_tbl.into_iter().enumerate() {
            res.extend(
                std::iter::repeat((b'a' + base_idx as u8) as char)
                    .take(cnt as usize)
                    .map(|c: char| c.to_string()),
            )
        }

        res
    }
}

pub fn raw_to_input(raw: &[&str]) -> Vec<String> {
    raw.into_iter().map(|s: &&str| s.to_string()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1002_1() {
        let input: Vec<String> = raw_to_input(&["bella", "label", "roller"]);
        let expected: Vec<String> = raw_to_input(&["e", "l", "l"]);

        assert_eq!(Solution::common_chars(input), expected);
    }

    #[test]
    fn test_1002_2() {
        let input: Vec<String> = raw_to_input(&["cool", "lock", "cook"]);
        let expected: Vec<String> = raw_to_input(&["c", "o"]);

        assert_eq!(Solution::common_chars(input), expected);
    }
}
