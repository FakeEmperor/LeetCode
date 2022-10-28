/*
 * @lc app=leetcode id=121 lang=rust
 *
 * [121] Best Time to Buy and Sell Stock
 */
pub struct Solution {}

// @lc code=start

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut profit = 0;
        let mut best_buy_price = 1_000_000;
        for price in prices {
            if best_buy_price > price {
                best_buy_price = price;
            }
            let diff = price - best_buy_price;
            if diff > profit {
                profit = diff
            }
            
        }
        profit
    }
}
// @lc code=end

