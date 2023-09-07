/*
 * @lc app=leetcode id=322 lang=rust
 *
 * [322] Coin Change
 */




pub struct Solution;

// @lc code=start
use std::iter::FromIterator;


impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let mut solutions: Vec<i32> = vec![0; amount as usize + 1];
        // let mut rcoins = Vec::from_iter(coins.iter().filter(|a| **a > 0).map(|a| *a as usize));
        // rcoins.sort_unstable_by(|a,b| b.cmp(a));

        for solving in 1..solutions.len() {
            let mut min_solution = i32::MAX;

            for coin in coins.iter() {
                if *coin as usize <= solving {
                    let child = solutions[solving - *coin as usize];
                    if child != -1 {
                        min_solution = min_solution.min(child);
                    }
                }
            }
            if min_solution == i32::MAX {
                solutions[solving] = -1;
            } else {
                solutions[solving] = min_solution + 1;
            }
        }
        
        solutions[amount as usize]
    }
}
// @lc code=end

