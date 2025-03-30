pub struct Solution;

impl Solution {
    pub fn min_costs(mut cost: Vec<i32>) -> Vec<i32> {
        let n: usize = cost.len();
        let mut ptr: usize = 0;
        let mut min_cost: i32 = i32::MAX;

        while ptr < n {
            let curr_cost: i32 = cost[ptr];

            if curr_cost < min_cost {
                min_cost = curr_cost;
            }

            cost[ptr] = min_cost;
            ptr += 1;
        }

        cost
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3502_1() {
        let cost: Vec<i32> = vec![5, 3, 4, 1, 3, 2];
        let expected: Vec<i32> = vec![5, 3, 3, 1, 1, 1];
        let res: Vec<i32> = Solution::min_costs(cost);

        assert_eq!(expected, res);
    }

    #[test]
    fn test_3502_2() {
        let cost: Vec<i32> = vec![1, 2, 4, 6, 7];
        let expected: Vec<i32> = vec![1, 1, 1, 1, 1];
        let res: Vec<i32> = Solution::min_costs(cost);

        assert_eq!(expected, res);
    }
}
