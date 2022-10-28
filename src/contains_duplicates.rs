/*
 * @lc app=leetcode id=217 lang=rust
 *
 * [217] Contains Duplicate
 */

struct Solution {}

// @lc code=start
use std::{collections::HashSet, iter::FromIterator};

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut s = HashSet::<i32, _>::with_capacity(nums.len());
        s.extend(nums.iter());
        s.len() != nums.len()
    }
}
// @lc code=end
