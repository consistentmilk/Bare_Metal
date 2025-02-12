pub struct Solution;

impl Solution {
    // Input: s = "daabcbaabcbc", part = "abc"
    // Output: "dab"
    // Explanation: The following operations are done:
    // - s = "daabcbaabcbc", remove "abc" starting at index 2, so s = "dabaabcbc".
    // - s = "dabaabcbc", remove "abc" starting at index 4, so s = "dababc".
    // - s = "dababc", remove "abc" starting at index 3, so s = "dab".
    // Now s has no occurrences of "abc".
    pub fn remove_occurrences(s: String, part: String) -> String {
        let pbytes: &[u8] = part.as_bytes();
        let mut svec: Vec<u8> = s.into_bytes();

        let plen: usize = pbytes.len();

        while let Some(pos) = svec.windows(plen).position(|w: &[u8]| w == pbytes) {
            svec.drain(pos..(pos + plen));
        }

        unsafe { String::from_utf8_unchecked(svec) }
    }

    // Knuth-Morris-Pratt Algorithm
    // So far, we have relied on a naive approach for pattern matching, where we slide the pattern (part) over the string (s) one character at a time and check for a match. For example, if s = "ABABDABACDABABCABAB" and part = "ABABCABAB", the naive approach compares part with every substring of s of the same length, often rechecking characters unnecessarily. Consider the scenario where the first four characters, "ABAB", match, but a mismatch occurs with the fifth character. In the naive approach, the pattern is shifted by just one character, and the comparison restarts from the beginning of part, rechecking "BAB" again. This results in redundant comparisons and inefficiency.
    // The Knuth-Morris-Pratt (KMP) algorithm optimizes this by using a longest prefix-suffix (LPS) array for the pattern. The LPS array helps determine how much of the pattern has been matched so far, allowing the algorithm to skip redundant comparisons. When a mismatch happens, instead of starting over from the beginning, we use the LPS array to shift the pattern by an appropriate amount.
    // For example, if we’ve matched "ABABC" but encounter a mismatch at the 6th character, the LPS value for "ABABC" is 1. We then shift the pattern by 4 characters (5 – 1) and continue matching. This avoids rechecking parts of the pattern we’ve already matched.
    // For example, consider the pattern part = "ABABCABAB". Let's see how we build up the LPS array in the slideshow below:
    // The LPS array allows the KMP algorithm to skip unnecessary comparisons when a mismatch occurs. When a mismatch happens, instead of starting over from the beginning of the pattern, the algorithm uses the LPS array to determine how much of the pattern has already been matched. It then shifts the pattern by an appropriate amount and continues matching.
    // For example, let’s say we’re matching part = "ABABCABAB" against s = "ABABDABACDABABCABAB". Suppose we’ve matched the first 4 characters ("ABAB") but encounter a mismatch at the 5th character. The LPS value for the prefix "ABAB" is 2, so we know that the first 2 characters of the pattern are already matched. Instead of starting over, we shift the pattern by 2 characters (length of the matched prefix minus the LPS value: 4 - 2 = 2) and continue matching. This skipping of unnecessary comparisons makes the KMP algorithm much more efficient.
    // The LPS array is built using a linear iterative approach. We initialize two pointers: current (to traverse part) and prefixLength (to track the length of the matching prefix-suffix). We then iterate through the pattern:
    // If the characters at current and prefixLength match, we increment both pointers and set lps[current] = prefixLength.
    // If they don’t match and prefixLength is not zero, we backtrack prefixLength to lps[prefixLength - 1].
    // If they don’t match and prefixLength is zero, we set lps[current] = 0 and increment current.
    // Finally, we process each character of s while using the LPS array to track how much of part has been matched. We iterate over s and when a complete match is found, we remove the matched substring from the stack. If a mismatch occurs, we use the LPS array to backtrack and continue matching.
    // After processing all characters of s, the stack contains the characters of s with all occurrences of part removed. We convert the stack into a string by popping characters and reversing the result (since stacks are last-in-first-out). We return this result as our answer.

    // Algorithm
    // Call the helper method computeLongestPrefixSuffix with the substring part to calculate the Longest Prefix Suffix (LPS) array.
    // Create a stack charStack to store characters of the string s as they are processed.
    // Declare an array patternIndexes of size s.length() + 1 to keep track of the pattern index for each character in the stack.
    // Use a for loop to iterate through each character in the string s. Also, maintain a variable patternIndex to track the current position in the substring part.
    // Push the current character onto the stack.
    // If the current character matches the character at patternIndex in part:
    // Increment patternIndex and store it in patternIndexes[charStack.size()].
    // If patternIndex equals the length of part, the pattern is fully matched:
    // Pop part.length() characters from the stack to remove the matched pattern.
    // Reset patternIndex to patternIndexes[charStack.size()] if the stack is not empty, otherwise set it to 0.
    // If the current character does not match the character at patternIndex in part:
    // If patternIndex is not 0, backtrack by setting patternIndex to lps[patternIndex - 1] and decrement strIndex to reprocess the current character.
    // If patternIndex is 0, set patternIndexes[charStack.size()] to 0.
    // Initialize result to construct the result string from the remaining characters in the stack.
    // Reverse the constructed string and return it as the output.
    // Helper method computeLongestPrefixSuffix(pattern)
    // Create an array lps of size equal to the length of the pattern part to store the lengths of the longest proper prefix which is also a suffix.
    // Use a for loop to traverse the pattern part starting from index 1. Maintain a variable prefixLength to track the length of the longest prefix-suffix.
    // If the character at the current position matches the character at prefixLength:
    // Increment prefixLength and store it in lps[current].
    // Proceed to the next character.
    // Else if the characters do not match and prefixLength is non-zero:
    // Backtrack to the previous longest prefix-suffix using the LPS array.
    // If no match is found and prefixLength is zero, set lps[current] to zero and proceed to the next character.
    // Return the fully constructed lps array.
    // Complexity Analysis
    // Let n be the length of the string s, and m be the length of the substring part.

    // Time complexity: O(n+m) for Java and Python3, O(n
    // 2
    //  +m) for C++
    // The algorithm consists of two main components: the preprocessing step to compute the KMP longest prefix-suffix (lps) array and the traversal of the string s.
    // The preprocessing step takes O(m) time, as the lps array is computed for the pattern part.
    // The traversal of s uses a stack and performs efficient pattern matching with the help of the lps array. Each character in s is processed once, and backtracking in the pattern matching is guided by the lps array, ensuring that each character is examined only a constant number of times. Thus, the traversal takes O(n) time.
    // Combining these two components, the overall time complexity is O(n+m).
    // However, the result construction step in the C++ solution has a time complexity of O(n
    // 2
    //  ) due to repeated string modifications. As a result, the overall time complexity of the C++ solution becomes O(n
    // 2
    //  +m).
    // Space complexity: O(n+m)
    // The primary space usage comes from the stack, which can store up to n characters in the worst case if no matches are removed. Additionally, the pattern matching indices array requires O(n) space, and the lps array used for KMP preprocessing requires O(m) space. These components together result in a total space complexity of O(n+m).
    pub fn remove_occurrences_kmp(s: String, part: String) -> String {
        let part_bytes: &[u8] = part.as_bytes();
        let part_len: usize = part_bytes.len();
        let kmp_lps: Vec<usize> = Self::compute_longest_prefix_suffix(part_bytes);

        let mut char_stack: Vec<u8> = Vec::with_capacity(s.len()); // Preallocate buffer
        let mut pattern_indexes: Vec<usize> = vec![0; s.len() + 1]; // Track pattern indices

        let mut str_index: usize = 0;
        let mut pattern_index: usize = 0;
        let s_bytes: &[u8] = s.as_bytes();

        while str_index < s_bytes.len() {
            let current_char: u8 = s_bytes[str_index];
            char_stack.push(current_char);

            if current_char == part_bytes[pattern_index] {
                pattern_indexes[char_stack.len()] = pattern_index + 1;
                pattern_index += 1;

                if pattern_index == part_len {
                    // Remove the matched pattern by truncating the stack
                    char_stack.truncate(char_stack.len() - part_len);

                    // Restore pattern index for the next potential match
                    pattern_index = if char_stack.is_empty() {
                        0
                    } else {
                        pattern_indexes[char_stack.len()]
                    };
                }
            } else {
                if pattern_index != 0 {
                    // Backtrack using KMP LPS
                    str_index -= 1;
                    pattern_index = kmp_lps[pattern_index - 1];
                    char_stack.pop();
                } else {
                    // Reset pattern index tracking
                    pattern_indexes[char_stack.len()] = 0;
                }
            }

            str_index += 1;
        }

        // Convert the stack back into a string
        String::from_utf8(char_stack).unwrap()
    }

    fn compute_longest_prefix_suffix(pattern: &[u8]) -> Vec<usize> {
        let mut lps: Vec<usize> = vec![0; pattern.len()];
        let mut current: usize = 1;
        let mut prefix_length: usize = 0;

        while current < pattern.len() {
            if pattern[current] == pattern[prefix_length] {
                prefix_length += 1;
                lps[current] = prefix_length;
                current += 1;
            } else if prefix_length != 0 {
                prefix_length = lps[prefix_length - 1];
            } else {
                lps[current] = 0;
                current += 1;
            }
        }

        lps
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1910_1() {
        let s: String = "daabcbaabcbc".into();
        let part: String = "abc".into();
        let expected: String = "dab".into();

        assert_eq!(expected, Solution::remove_occurrences(s, part));
    }

    #[test]
    fn test_1910_2() {
        let s: String = "axxxxyyyyb".into();
        let part: String = "xy".into();
        let expected: String = "ab".into();

        assert_eq!(expected, Solution::remove_occurrences(s, part));
    }
}
