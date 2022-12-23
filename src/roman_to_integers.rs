/*
 * @lc app=leetcode id=13 lang=rust
 *
 * [13] Roman to Integer
 */

pub struct Solution {}

// @lc code=start

use std::collections::HashMap;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let romans: HashMap<char, i32> = [
            ('I', 1),
            ('V', 5),
            ('X', 10),
            ('L', 50),
            ('C', 100),
            ('D', 500),
            ('M', 1000),
        ]
        .iter()
        .cloned()
        .collect();

        let mut total_sum = 0;
        let mut used = false;

        for idx in 0..s.len() {
            if used { used = false; continue; }
            let cur =  *romans.get(&s.chars().nth(idx).unwrap()).unwrap();
            if idx < s.len() - 1 {
                let next = *romans.get(&s.chars().nth(idx+1).unwrap()).unwrap();
                if next > cur {
                    total_sum += next - cur;
                    used = true;
                } else {
                    total_sum += cur;
                }
            } else {
                total_sum += cur;
            }
            
            
        }
        total_sum
    }
}
// @lc code=end
