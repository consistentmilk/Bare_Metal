use crate::euler::number_theory::NumberTheory;

pub struct Solution;

impl Solution {
    pub fn smallest_multiple(n: u128) -> u128 {
        let mut result: u128 = 1;

        for i in 1..=n {
            result = NumberTheory::lcm(result, i);
        }

        result
    }
}
