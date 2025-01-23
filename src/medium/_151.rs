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
        if s.is_empty() {
            return String::new();
        }

        let mut a: Vec<char> = s.chars().collect();
        let n: usize = a.len();

        // Step 1: Reverse the whole string
        Self::reverse(&mut a, 0, n - 1);
        // Step 2: Reverse each word
        Self::reverse_words_inplace(&mut a, n);
        // Step 3: Clean up spaces
        Self::clean_spaces(&mut a, n)
    }

    fn reverse_words_inplace(a: &mut [char], n: usize) {
        let mut i: usize = 0;
        let mut j: usize = 0;

        while i < n {
            while i < j || (i < n && a[i] == ' ') {
                i += 1; // Skip spaces
            }

            while j < i || (j < n && a[j] != ' ') {
                j += 1; // Skip non-spaces
            }

            Self::reverse(a, i, j - 1); // Reverse the word
        }
    }

    // Trim leading, trailing, and multiple spaces
    fn clean_spaces(a: &mut [char], n: usize) -> String {
        let mut i: usize = 0;
        let mut j: usize = 0;

        while j < n {
            while j < n && a[j] == ' ' {
                j += 1; // Skip spaces
            }
            while j < n && a[j] != ' ' {
                a[i] = a[j];
                i += 1;
                j += 1; // Keep non-spaces
            }
            while j < n && a[j] == ' ' {
                j += 1; // Skip spaces
            }
            if j < n {
                a[i] = ' ';
                i += 1; // Keep only one space
            }
        }

        a[..i].iter().collect()
    }

    // Reverse a[] from a[i] to a[j]
    fn reverse(a: &mut [char], mut i: usize, mut j: usize) {
        while i < j {
            a.swap(i, j);
            i += 1;
            j -= 1;
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_151_one() {}
}
