pub struct Solution;

// Given an integer array data representing the data, return whether it is a valid UTF-8 encoding
// (i.e. it translates to a sequence of valid UTF-8 encoded characters).

// A character in UTF8 can be from 1 to 4 bytes long, subjected to the following rules:

// For a 1-byte character, the first bit is a 0, followed by its Unicode code.
// For an n-bytes character, the first n bits are all one's, the n + 1 bit is 0, followed by n - 1
// bytes with the most significant 2 bits being 10.
// This is how the UTF-8 encoding would work:

//      Number of Bytes   |        UTF-8 Octet Sequence
//                        |              (binary)
//    --------------------+-----------------------------------------
//             1          |   0xxxxxxx
//             2          |   110xxxxx 10xxxxxx
//             3          |   1110xxxx 10xxxxxx 10xxxxxx
//             4          |   11110xxx 10xxxxxx 10xxxxxx 10xxxxxx
// x denotes a bit in the binary form of a byte that may be either 0 or 1.

// Note: The input is an array of integers. Only the least significant 8 bits of each integer is
//used to store the data. This means each integer represents only 1 byte of data.

// Example 1:
// Input: data = [197,130,1]
// Output: true
// Explanation: data represents the octet sequence: 11000101 10000010 00000001.
// It is a valid utf-8 encoding for a 2-bytes character followed by a 1-byte character.

// Example 2:
// Input: data = [235,140,4]
// Output: false
// Explanation: data represented the octet sequence: 11101011 10001100 00000100.
// The first 3 bits are all one's and the 4th bit is 0 means it is a 3-bytes character.
// The next byte is a continuation byte which starts with 10 and that's correct.
// But the second continuation byte does not start with 10, so it is invalid.

// Constraints:
// 1 <= data.length <= 2 * 104
// 0 <= data[i] <= 255

impl Solution {
    pub fn valid_utf8(data: Vec<i32>) -> bool {
        let mut n_bytes: i32 = 0;

        let mask1: i32 = 1 << 7;
        let mask2: i32 = 1 << 6;

        for num in data {
            let mut mask: i32 = 1 << 7;

            if n_bytes == 0 {
                // check how many bytes there are
                while mask & num != 0 {
                    n_bytes += 1;
                    mask = mask >> 1;
                }

                // if n_bytes is zero, it implies that this is a valid 1 byte UTF-8 character
                if n_bytes == 0 {
                    continue;
                }

                // If n_bytes is 1 or greater than 4, then this is not a valid UTF-8 encoded character
                if n_bytes == 1 || n_bytes > 4 {
                    return false;
                }
            } else {
                if !((mask1 & num != 0) && (mask2 & num == 0)) {
                    return false;
                }
            }

            n_bytes -= 1;
        }

        n_bytes == 0
    }
}

impl Solution {
    pub fn valid_utf8_alt(data: Vec<i32>) -> bool {
        let mut count: i32 = 0;

        for num in data {
            if count == 0 {
                if num >> 5 == 0b110 {
                    count = 1;
                } else if num >> 4 == 0b1110 {
                    count = 2;
                } else if num >> 3 == 0b11110 {
                    count = 3;
                } else if num >> 7 != 0 {
                    return false;
                }
            } else {
                if num >> 6 != 0b10 {
                    return false;
                }

                count -= 1;
            }
        }

        count == 0
    }
}

#[derive(Clone, Copy, PartialEq)]
enum State {
    ExpectingHeader,
    ExpectingOne,
    ExpectingTwo,
    ExpectingThree,
    Error,
    Invalid,
}

impl Solution {
    /// State Machine based solution
    pub fn valid_utf8_sm(data: Vec<i32>) -> bool {
        data.into_iter()
            .map(|i: i32| (i as u8).leading_ones())
            .scan(State::ExpectingHeader, |state: &mut State, lo: u32| {
                *state = match (*state, lo) {
                    (State::ExpectingHeader, 0) => State::ExpectingHeader,
                    (State::ExpectingHeader, 2) => State::ExpectingOne,
                    (State::ExpectingHeader, 3) => State::ExpectingTwo,
                    (State::ExpectingHeader, 4) => State::ExpectingThree,
                    (State::ExpectingOne, 1) => State::ExpectingHeader,
                    (State::ExpectingTwo, 1) => State::ExpectingOne,
                    (State::ExpectingThree, 1) => State::ExpectingTwo,
                    (State::Error, _) => State::Invalid,
                    _ => State::Error,
                };

                if *state == State::Invalid {
                    None
                } else {
                    Some(*state)
                }
            })
            .map(|state: State| state == State::ExpectingHeader)
            .last()
            .unwrap_or(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_393_one() {
        let test = vec![197, 130, 1];
        assert!(Solution::valid_utf8(test));
    }

    #[test]
    fn test_393_two() {
        let test = vec![235, 140, 4];
        assert!(!Solution::valid_utf8(test));
    }

    #[test]
    fn test_393_three() {
        let test = vec![250, 145, 145, 145, 145];
        assert!(!Solution::valid_utf8(test));
    }
}
