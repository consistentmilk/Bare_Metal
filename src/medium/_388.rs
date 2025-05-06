/*
Intuition:
- A single byte-wise pass lets us parse depth, name, and file-flag
  without extra scans.
- Tracking cumulative lengths per depth avoids rebuilding paths.
- Detect files by noting a '.' during the name scan.

Algorithm:
1. Convert input to bytes.
2. Initialize depth_len[0] = 0.
3. While bytes remain:
   a. Count leading '\t' for depth.
   b. Scan name bytes, detect '.', count name length.
   c. Skip newline.
   d. Resize depth_len to lev+2 if needed.
   e. Compute cur_len = parent + (lev>0?1:0) + name_len.
   f. Store depth_len[lev+1] = cur_len.
   g. If file, update max_len.

Time Complexity:
- O(N) overall, one pass over all N bytes.

Space Complexity:
- O(D) where D is max directory depth (depth_len grows per depth).
*/

pub struct Solution;

impl Solution {
    pub fn length_longest_path(input: String) -> i32 {
        // Convert the input string to a byte slice
        let bytes: &[u8] = input.as_bytes();

        // Byte‐index pointer for iteration
        let mut i: usize = 0;

        // Record the maximum file path length found
        let mut max_len: usize = 0;

        // Store cumulative lengths at each depth level
        let mut depth_len: Vec<usize> = Vec::with_capacity(64);

        // Depth 0 (root) has cumulative length 0
        depth_len.push(0);

        // Process until we reach the end of the byte slice
        while i < bytes.len() {
            // Count '\t' bytes to determine current depth level
            let mut lev: usize = 0;

            while i < bytes.len() && bytes[i] == b'\t' {
                lev += 1;
                i += 1;
            }

            // Flag to mark if the entry is a file (contains '.')
            let mut is_file: bool = false;

            // Count characters in the name portion
            let mut name_len: usize = 0;

            while i < bytes.len() && bytes[i] != b'\n' {
                if bytes[i] == b'.' {
                    is_file = true;
                }

                name_len += 1;
                i += 1;
            }

            // Skip the newline character (if present)
            if i < bytes.len() {
                i += 1;
            }

            // Ensure we have space for depth index lev+1
            if depth_len.len() <= lev + 1 {
                depth_len.resize(lev + 2, 0);
            }

            // Compute cumulative length: parent + '/' + name_len
            let cur_len: usize = if lev == 0 {
                name_len
            } else {
                depth_len[lev] + 1 + name_len
            };

            // Store cumulative length for this depth + 1
            depth_len[lev + 1] = cur_len;

            // If this entry is a file, update the maximum length
            if is_file && cur_len > max_len {
                max_len = cur_len;
            }
        }

        // Return the result as i32
        max_len as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_388_empty_input_1() {
        let input = String::new();
        assert_eq!(Solution::length_longest_path(input), 0);
    }

    #[test]
    fn test_388_only_directories_2() {
        let input = String::from("dir\n\tsubdir\n\tsubsubdir");
        assert_eq!(Solution::length_longest_path(input), 0);
    }

    #[test]
    fn test_388_single_file_in_root_3() {
        let input = String::from("file.txt");
        assert_eq!(Solution::length_longest_path(input), 8);
    }

    #[test]
    fn test_388_nested_example_4() {
        let input = String::from("dir\n\tsubdir1\n\tsubdir2\n\t\tfile.ext");
        assert_eq!(Solution::length_longest_path(input), 20);
    }

    #[test]
    fn test_388_complex_nested_example_5() {
        let input = String::from(
            "dir\n\tsubdir1\n\t\tfile1.ext\n\t\tsubsubdir1\n\t\
             subdir2\n\t\tsubsubdir2\n\t\t\tfile2.ext",
        );
        assert_eq!(Solution::length_longest_path(input), 32);
    }
}
