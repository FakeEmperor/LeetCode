/*
 * @lc app=leetcode id=350 lang=rust
 *
 * [350] Intersection of Two Arrays II
 */

pub struct Solution {}

// @lc code=start

use std::{collections::HashMap, iter, ops::AddAssign};
impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut positions1 = HashMap::<i32, i32>::with_capacity(nums1.len());
        let mut positions2 = HashMap::<i32, i32>::with_capacity(nums2.len());
        for value in nums1.iter() {
            (*positions1.entry(*value).or_insert(0)).add_assign(1);
        }
        for value in nums2.iter() {
            if positions1.contains_key(value) {
                (positions2.entry(*value).or_insert(0)).add_assign(1);
            }
        }
        let mut intersection = vec![];
        for (key, repetitions2) in positions2.iter() {
            intersection.extend(
                iter::repeat(key).take(
                    *positions1
                        .entry(*key)
                        .and_modify(|e| *e = (*e).min(*repetitions2))
                        .or_default() as usize,
                ),
            );
        }
        intersection
    }
}
// @lc code=end
