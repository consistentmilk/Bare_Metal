pub struct Solution {}

// You are given an array prices where prices[i] is the price of a given stock on the ith day.
// You want to maximize your profit by choosing a single day to buy one stock and choosing a different day in the future to sell that stock.
// Return the maximum profit you can achieve from this transaction. If you cannot achieve any profit, return 0.

// Example 1:
// Input: prices = [7,1,5,3,6,4]
// Output: 5
// Explanation: Buy on day 2 (price = 1) and sell on day 5 (price = 6), profit = 6-1 = 5.
// Note that buying on day 2 and selling on day 1 is not allowed because you must buy before you sell.

// Example 2:
// Input: prices = [7,6,4,3,1]
// Output: 0
// Explanation: In this case, no transactions are done and the max profit = 0.

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut buy: i32 = prices[0];
        let mut profit: i32 = 0;

        for i in 1..prices.len() {
            if prices[i] < buy {
                buy = prices[i];
            } else if prices[i] - buy > profit {
                profit = prices[i] - buy;
            }
        }

        profit
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_121_one() {
        let prices: Vec<i32> = vec![7, 1, 5, 3, 6, 4];
        let expected: i32 = 5;

        assert_eq!(Solution::max_profit(prices), expected);
    }

    #[test]
    fn test_121_two() {
        let prices: Vec<i32> = vec![7, 6, 4, 3, 1];
        let expected: i32 = 0;

        assert_eq!(Solution::max_profit(prices), expected);
    }
}
