pub struct Solution;

impl Solution {
    pub fn reverse_words(s: String) -> String {
        let str_vec: Vec<&str> = s.trim().split_whitespace().rev().collect();

        str_vec.join(" ").into()
    }

    pub fn _reverse_words_alt(s: String) -> String {
        s.split_whitespace().rev().collect::<Vec<&str>>().join(" ")
    }
}

pub struct SolutionAlt;

/// Uses two pointer method
impl SolutionAlt {
    pub fn reverse_words(s: String) -> String {
        // Convert the string into a mutable Vec<u8> for in-place manipulation
        let mut s: Vec<u8> = s.into_bytes();

        // Trim leading and trailing spaces
        let trimmed_len: usize = Self::trim_spaces(&mut s);

        // Reverse the entire string
        Self::reverse_range(&mut s, 0, trimmed_len);

        // Reverse each word individually
        Self::reverse_each_word(&mut s, trimmed_len);

        // Convert the Vec<u8> back to a String
        unsafe { String::from_utf8_unchecked(s) }
    }

    // Helper function to trim leading and trailing spaces
    fn trim_spaces(s: &mut Vec<u8>) -> usize {
        let mut write_idx: usize = 0;
        let mut read_idx: usize = 0;
        let n = s.len();

        // Skip leading spaces
        while read_idx < n && s[read_idx] == b' ' {
            read_idx += 1;
        }

        // Copy non-space characters and ensure single spaces between words
        while read_idx < n {
            if s[read_idx] != b' ' {
                s[write_idx] = s[read_idx];
                write_idx += 1;
            } else if write_idx > 0 && s[write_idx - 1] != b' ' {
                s[write_idx] = b' ';
                write_idx += 1;
            }
            read_idx += 1;
        }

        // Remove trailing space if any
        if write_idx > 0 && s[write_idx - 1] == b' ' {
            write_idx -= 1;
        }

        // Truncate the vector to the new length
        s.truncate(write_idx);

        write_idx
    }

    // Helper function to reverse a range of the vector
    fn reverse_range(s: &mut Vec<u8>, mut left: usize, mut right: usize) {
        while left < right {
            s.swap(left, right - 1);
            left += 1;
            right -= 1;
        }
    }

    // Helper function to reverse each word in the vector
    fn reverse_each_word(s: &mut Vec<u8>, n: usize) {
        let mut start: usize = 0;
        let mut end: usize = 0;

        while end < n {
            // Find the end of the current word
            while end < n && s[end] != b' ' {
                end += 1;
            }

            // Reverse the current word
            Self::reverse_range(s, start, end);

            // Move to the next word
            start = end + 1;
            end = start;
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_151_one() {}
}
