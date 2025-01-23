pub struct Solution {}

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;

        let mut map: HashMap<i32, i32> = HashMap::new();

        let mut major: i32 = nums[0];
        let mut major_count: i32 = 1;

        for num in nums {
            if let Some(count) = map.get_mut(&num) {
                *count += 1;

                if count > &mut major_count {
                    major = num;
                    major_count = *count;
                }
            } else {
                map.insert(num, 1);
            }
        }

        major
    }

    /// Intuition
    // The Boyer-Moore Majority Vote Algorithm is designed to find the majority element in an array efficiently. The majority element is the one that appears more than ⌊n/2⌋ times, where n is the size of the array. The key insight is that if we cancel out each occurrence of an element with all the other elements that are different from it, the majority element will still remain in the last position.

    // Understanding the Algorithm
    // The idea is to scan through the elements of the sequence and keep track of a candidate element. When a new element is encountered, if it matches the candidate, we increase a counter; if it doesn't match, we decrease the counter. If the counter becomes zero, we choose the current element as the new candidate.

    // The algorithm guarantees that if there is a majority element (an element that appears more than half the time), it will be the final candidate. After scanning the entire sequence, we perform a second pass to confirm that the candidate is indeed the majority element.

    // This algorithm is efficient because it eliminates pairs of different elements, gradually revealing the majority element as the final candidate.This can be expressed in pseudocode as the following steps:

    // Initialize an element m and a counter c with c = 0
    // For each element x of the input sequence:
    // If c = 0, then assign m = x and c = 1
    // else if m = x, then assign c = c + 1
    // else assign c = c − 1
    // Return m

    // Approach
    // The algorithm uses two variables, m and c.

    // m : Represents the current candidate for the majority element.
    // c : Represents a counter for the occurrences of the current candidate.
    // The algorithm iterates through the array, and for each element:

    // If ccc is zero, it sets the current element as the candidate mmm and resets ccc to 1.
    // If the current element is the same as the candidate mmm, it increments ccc.
    // If the current element is different from the candidate mmm, it decrements ccc.
    // This way, as the algorithm progresses, it keeps a track of the potential majority element. If there is a majority element in the array, it will be the final value of m after the iteration.

    // Complexity
    // Time complexity:
    // The algorithm iterates through the array once, so the time complexity is O(n), where n is the size of the array.

    // Space complexity:
    // The algorithm uses only a constant amount of space (two variables), so the space complexity is O(1). No additional space is proportional to the input size.
    pub fn majority_element_alt(nums: Vec<i32>) -> i32 {
        let mut c: i32 = 0;
        let mut m: i32 = 0;

        for x in nums {
            if c == 0 {
                m = x;
                c = 1;
            } else if x == m {
                c += 1;
            } else {
                c -= 1;
            }
        }

        m
    }
}
