use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        // Input is ASCII => chars are bytes
        let s: &[u8] = s.as_bytes();
        let n: usize = s.len();
        let k: usize = words[0].len();

        // Early exit if word length is greater than length of s
        if k > n {
            return vec![];
        }

        // Build map from word (as bytes) to (frequency of word in words, recorded frequency in window)
        let mut map: HashMap<&[u8], (usize, usize)> =
            words
                .iter()
                .fold(HashMap::<&[u8], (usize, usize)>::new(), |mut map, word| {
                    map.entry(word.as_bytes()).or_default().0 += 1;
                    map
                });

        // Flag to tell if map is reset to avoid resetting a map that is already reset
        let mut map_is_reset: bool = true;
        let mut rez: Vec<i32> = vec![];

        // We have to run the sliding window algorithm with every offset in the word length
        for i in 0..k {
            // Reset window word frequency if needed
            if !map_is_reset {
                map.iter_mut().for_each(|(_, value)| value.1 = 0);
                map_is_reset = true;
            }

            // Initialize empty window at start position
            let (mut lo, mut hi) = (i, i);
            while hi <= n - k {
                match map.get_mut(&s[hi..hi + k]) {
                    None => {
                        // No match at current hi position - reset window at next word boundary
                        hi += k;
                        lo = hi;

                        if !map_is_reset {
                            map.iter_mut().for_each(|(_, value)| value.1 = 0);
                            map_is_reset = true;
                        }
                    }
                    Some(hi_value) => {
                        // Word found - update recorded word frequency
                        hi_value.1 += 1;
                        hi += k;
                        map_is_reset = false;

                        // If word frequency in current window is too high we have to slide lo
                        // forward until we decrease the frequency of the word encountered at hi
                        // back to the upper limit.
                        if hi_value.1 > hi_value.0 {
                            loop {
                                let lo_value = map.get_mut(&s[lo..lo + k]).unwrap();
                                lo += k;
                                lo_value.1 -= 1;

                                if lo_value.0 == lo_value.1 {
                                    break;
                                }
                            }
                        }
                    }
                }

                // The window always contains a valid count of words at this point, so if the window
                // is so large that it can contain all the words, lo is a match to be recorded in the
                // return value.
                if hi - lo == words.len() * k {
                    rez.push(lo as i32);
                    map.get_mut(&s[lo..lo + k]).unwrap().1 -= 1;
                    lo += k;
                }
            }
        }
        rez
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_substring() {
        let s = "barfoothefoobarman";
        let words = vec!["foo", "bar"]
            .iter()
            .map(|&s| s.to_string())
            .collect::<Vec<String>>();

        let expected: Vec<i32> = vec![0, 9];

        assert_eq!(Solution::find_substring(s.to_string(), words), expected);
    }
}
