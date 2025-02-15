pub struct Solution;

use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut store: BinaryHeap<Reverse<i32>> = BinaryHeap::<Reverse<i32>>::new();
        let mut count: i32 = 0;

        // Push only elements less than k into the heap
        for &num in nums.iter().filter(|num: &&i32| **num < k) {
            store.push(Reverse(num));
        }

        // Process the heap
        while store.len() >= 2 {
            let Reverse(num1) = store.pop().unwrap();
            let Reverse(num2) = store.pop().unwrap();

            // Combine the two smallest elements
            let new_num: i32 = num1.saturating_mul(2).saturating_add(num2);

            // If the combined value is still less than k, push it back
            if new_num < k {
                store.push(Reverse(new_num));
                count += 1;
            } else {
                // If the combined value is >= k, calculate remaining operations
                let remaining: i32 = store.len() as i32;
                return count + 1 + (remaining + 1) / 2;
            }
        }

        // Handle the case where only one element is left in the heap
        if let Some(Reverse(num)) = store.pop() {
            if num < k {
                count += 1;
            }
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3066_1() {
        let nums: Vec<i32> = vec![2, 11, 10, 1, 3];
        let k: i32 = 10;
        let expected: i32 = 2;

        assert_eq!(expected, Solution::min_operations(nums, k));
    }

    #[test]
    fn test_3066_2() {
        let nums: Vec<i32> = vec![1, 1, 2, 4, 9];
        let k: i32 = 20;
        let expected: i32 = 4;

        assert_eq!(expected, Solution::min_operations(nums, k));
    }
}
