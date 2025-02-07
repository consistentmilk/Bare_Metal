use std::collections::hash_map::Entry;
use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        let s_bytes: Vec<u8> = s.into_bytes();
        let t_bytes: Vec<u8> = t.into_bytes();
        let mut char_freq: HashMap<u8, u32> = HashMap::new();

        for char_byte in s_bytes {
            char_freq
                .entry(char_byte)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }

        for char_byte in t_bytes {
            match char_freq.entry(char_byte) {
                Entry::Occupied(mut count) => {
                    *count.get_mut() -= 1;

                    if *count.get() == 0 {
                        char_freq.remove(&char_byte);
                    }
                }

                Entry::Vacant(_) => {
                    return char_byte as char;
                }
            }
        }

        ' '
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_389_1() {
        let s: String = "abcd".to_string();
        let t: String = "abcde".to_string();
        let expected: char = 'e';
        let res: char = Solution::find_the_difference(s, t);

        assert_eq!(expected, res);
    }

    #[test]
    fn test_389_2() {
        let s: String = "".to_string();
        let t: String = "y".to_string();
        let expected: char = 'y';
        let res: char = Solution::find_the_difference(s, t);

        assert_eq!(expected, res);
    }
}
