pub struct Solution;

impl Solution {
    pub fn duplicate_zeros(arr: &mut Vec<i32>) {
        let mut possible_dups: usize = 0;
        let mut length: usize = arr.len() - 1;

        let mut left: usize = 0;
        while left <= length - possible_dups {
            if arr[left] == 0 {
                if left == length - possible_dups {
                    arr[length] = 0;
                    length -= 1;
                    break;
                }

                possible_dups += 1;
            }

            left += 1;
        }

        let last: usize = length - possible_dups;
        for i in (0..=last).rev() {
            if arr[i] == 0 {
                arr[i + possible_dups] = 0;
                possible_dups -= 1;
                arr[i + possible_dups] = 0;
            } else {
                arr[i + possible_dups] = arr[i];
            }
        }
    }
}
