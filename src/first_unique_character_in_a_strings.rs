/*
 * @lc app=leetcode id=387 lang=rust
 *
 * [387] First Unique Character in a String
 */


pub struct Solution {}

// @lc code=start
use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut m = HashMap::<char, usize>::with_capacity(26);
        let mut ban = HashSet::<char>::with_capacity(26);
        for (i, ch) in s.chars().enumerate() {
            if ban.contains(&ch) { continue; } 
            if m.contains_key(&ch) {
                m.remove(&ch);
                ban.insert(ch);
            } else {
                m.insert(ch, i);
            }
        }
        if m.is_empty() {
            -1
        } else {
            let mut idx = s.len();
            for (_, i) in m {
                idx = idx.min(i);
            }
            idx as i32
        }
    }
}
// @lc code=end

