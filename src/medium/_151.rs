pub struct Solution;

impl Solution {
    pub fn reverse_words(s: String) -> String {
        // Convert the string into a mutable Vec<u8> for in-place manipulation
        let mut char_bytes: Vec<u8> = s.into_bytes();

        // Trim leading, extra spaces between words and trailing spaces
        let trimmed_len: usize = Self::trim_spaces(&mut char_bytes);

        // At this point the extra spaces are removed, so reverse the full vec
        Self::reverse_range(&mut char_bytes, 0, trimmed_len);

        // Reverse each word again
        Self::reverse_each_word(&mut char_bytes, trimmed_len);

        unsafe { String::from_utf8_unchecked(char_bytes) }
    }

    fn trim_spaces(char_bytes: &mut Vec<u8>) -> usize {
        let mut read_idx: usize = 0;
        let mut write_idx: usize = 0;
        let n: usize = char_bytes.len();

        while read_idx < n && char_bytes[read_idx] == b' ' {
            read_idx += 1;
        }

        while read_idx < n {
            if char_bytes[read_idx] != b' ' {
                char_bytes[write_idx] = char_bytes[read_idx];
                write_idx += 1;
            } else if write_idx > 0 && char_bytes[write_idx - 1] != b' ' {
                char_bytes[write_idx] = b' ';
                write_idx += 1;
            }

            read_idx += 1;
        }

        if write_idx > 0 && char_bytes[write_idx - 1] == b' ' {
            write_idx -= 1;
        }

        char_bytes.truncate(write_idx);

        write_idx
    }

    // Reverse a range of the vector
    fn reverse_range(char_bytes: &mut Vec<u8>, mut lp: usize, mut rp: usize) {
        while lp < rp {
            char_bytes.swap(lp, rp - 1);
            lp += 1;
            rp -= 1;
        }
    }

    // Reverse each word in the vector
    fn reverse_each_word(char_bytes: &mut Vec<u8>, n: usize) {
        let mut lp: usize = 0;
        let mut rp: usize = 0;

        while rp < n {
            while rp < n && char_bytes[rp] != b' ' {
                rp += 1;
            }

            Self::reverse_range(char_bytes, lp, rp);

            lp = rp + 1;
            rp = lp;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_151_1() {
        let s: String = "the sky is blue".to_string();
        let expected: String = "blue is sky the".to_string();

        assert_eq!(expected, Solution::reverse_words(s));
    }

    #[test]
    fn test_151_2() {
        let s: String = "  hello world  ".to_string();
        let expected: String = "world hello".to_string();

        assert_eq!(expected, Solution::reverse_words(s));
    }

    #[test]
    fn test_151_3() {
        let s: String = "a good   example".to_string();
        let expected: String = "example good a".to_string();

        assert_eq!(expected, Solution::reverse_words(s));
    }
}
