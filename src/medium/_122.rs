use std::cmp::max;

pub struct Solution {}

// You are given an integer array prices where prices[i] is the price of a given stock on the ith day.
// On each day, you may decide to buy and/or sell the stock. You can only hold at most one share of the stock at any time. However, you can buy it then immediately sell it on the same day.
// Find and return the maximum profit you can achieve.

// Example 1:
// Input: prices = [7,1,5,3,6,4]
// Output: 7
// Explanation: Buy on day 2 (price = 1) and sell on day 3 (price = 5), profit = 5-1 = 4.
// Then buy on day 4 (price = 3) and sell on day 5 (price = 6), profit = 6-3 = 3.
// Total profit is 4 + 3 = 7.

// Example 2:
// Input: prices = [1,2,3,4,5]
// Output: 4
// Explanation: Buy on day 1 (price = 1) and sell on day 5 (price = 5), profit = 5-1 = 4.
// Total profit is 4.

// Example 3:
// Input: prices = [7,6,4,3,1]
// Output: 0
// Explanation: There is no way to make a positive profit, so we never buy the stock to achieve the maximum profit of 0.

// Constraints:
// 1 <= prices.length <= 3 * 104
// 0 <= prices[i] <= 104

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        prices.windows(2).map(|s: &[i32]| max(s[1] - s[0], 0)).sum()
    }

    pub fn max_profit_alt(prices: Vec<i32>) -> i32 {
        let mut profit: i32 = 0;
        let mut buy: i32 = prices[0];
        let days: usize = prices.len();

        for i in 1..days {
            if buy < prices[i] {
                profit += prices[i] - buy;
            }

            buy = prices[i];
        }

        profit
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_122_one() {
        let prices: Vec<i32> = vec![7, 1, 5, 3, 6, 4];
        let expected: i32 = 7;

        assert_eq!(Solution::max_profit_alt(prices), expected);
    }

    #[test]
    fn test_122_two() {
        let prices: Vec<i32> = vec![1, 2, 3, 4, 5];
        let expected: i32 = 4;

        assert_eq!(Solution::max_profit_alt(prices), expected);
    }

    #[test]
    fn test_122_three() {
        let prices: Vec<i32> = vec![7, 6, 4, 3, 1];
        let expected: i32 = 0;

        assert_eq!(Solution::max_profit_alt(prices), expected);
    }
}
