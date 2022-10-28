/*
 * @lc app=leetcode id=1 lang=rust
 *
 * [1] Two Sum
 */

pub struct Solution {}

// @lc code=start
use std::{collections::HashMap, iter::FromIterator};

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut num_map = HashMap::<i32, Vec<usize>>::with_capacity(nums.len());
        for (idx, value) in nums.iter().enumerate() {
            if num_map.contains_key(value) {
                num_map.get_mut(value).unwrap().push(idx);
            } else {
                num_map.insert(*value, vec![idx]);
            }
        }

        for (value, idx) in num_map.iter() {
            let remainder = target - *value;
            match num_map.get(&remainder) {
                Some(found) => {
                    if idx as *const _ != found as *const _ {
                        return vec![found[0] as i32, idx[0] as i32];
                    } else if found.len() > 1 {
                        return Vec::from_iter(found.iter().take(2).map(|x| *x as i32));
                    }
                }
                _ => {}
            }
        }

        vec![0, 0]
    }
}
// @lc code=end
