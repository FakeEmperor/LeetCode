
/*
* @lc app=leetcode id=15 lang=rust
*
* [15] 3Sum
*/
pub struct Solution;

// @lc code=start
use std::{collections::{HashMap, HashSet}, iter::FromIterator};

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort_unstable();
        let mut triplets: Vec<Vec<i32>> = vec![];

        for (i, x1) in nums.iter().enumerate() {
            let x1 = *x1;
            
            if x1 > 0 { break; }  // look only non-positives
            if i > 0 && x1 == nums[i-1] { continue; } // skip same numbers

            let mut j = i + 1;
            let mut k = nums.len() - 1;
            
            while j < k {
                let sum = x1 + nums[j] + nums[k];
                if sum == 0 {
                    triplets.push(vec![x1, nums[j], nums[k]]);
                    j += 1;
                    while j < nums.len() && nums[j] == nums[j-1] { j+= 1; }
                } else if sum < 0 {
                    j += 1;
                } else {
                    k -= 1;
                }
            }
        }
        return triplets;
    }
}
// @lc code=end

