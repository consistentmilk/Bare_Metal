pub struct Solution;

impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let n: usize = cost.len();

        let mut curr = cost[0];
        let mut prev = cost[1];

        for i in 2..n {
            let min_cost = cost[i] + std::cmp::min(curr, prev);
            curr = prev;
            prev = min_cost;
        }

        std::cmp::min(curr, prev)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_746_1() {
        let cost: Vec<i32> = vec![10, 15, 20];
        let expected: i32 = 15;
        assert_eq!(expected, Solution::min_cost_climbing_stairs(cost));
    }

    #[test]
    fn test_746_2() {
        let cost: Vec<i32> = vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1];
        let expected: i32 = 6;
        assert_eq!(expected, Solution::min_cost_climbing_stairs(cost));
    }
}
