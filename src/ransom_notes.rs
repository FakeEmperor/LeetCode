/*
 * @lc app=leetcode id=383 lang=rust
 *
 * [383] Ransom Note
 */


pub struct Solution {}

// @lc code=start
use std::{ops::{SubAssign}};

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut alphabet = [0; 26];
        let achar = 'a' as usize;
        for ch in magazine.chars() {
            alphabet[(ch as usize) - achar] += 1;
        }
        for ch in ransom_note.chars() {
           let val = &mut alphabet[(ch as usize) - achar];
           if *val == 0 { return false; }
           val.sub_assign(1);
        }
        true
    }
}
// @lc code=end

