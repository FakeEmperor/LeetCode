/*
 * @lc app=leetcode id=20 lang=rust
 *
 * [20] Valid Parentheses
 */

pub struct Solution {}

// @lc code=start
use std::collections::VecDeque;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        if s.is_empty() { return true; }
        let mut parens = VecDeque::<char>::with_capacity(s.len()-1);
        for ch in s.chars() {
            match ch {
                '(' | '[' | '{' => {
                    parens.push_back(ch);
                },
                ')' => {
                    if parens.is_empty() || parens[parens.len()-1] != '(' {return false;}
                    parens.pop_back();
                },
                '}' => {
                    if parens.is_empty() || parens[parens.len()-1] != '{' {return false;}
                    parens.pop_back();
                },
                ']' => {
                    if parens.is_empty() || parens[parens.len()-1] != '[' {return false;}
                    parens.pop_back();
                }
                _ => {unreachable!();}
            }
        }
        parens.is_empty()
    }
}
// @lc code=end

