/*
 * @lc app=leetcode id=20 lang=rust
 *
 * [20] Valid Parentheses
 */

pub struct Solution;

// @lc code=start
impl Solution {
    pub fn is_valid(s: String) -> bool {
        if s.len() % 2 == 1 { return false; }
        let bytes = s.as_bytes();
        if !matches!(bytes[0], b'(' | b'[' | b'{') { return false; }

        let mut parens: Vec<u8> = vec![];
        for b in bytes {
            let is_open = match b {
                b'(' | b'[' | b'{' => true,
                _ => false
            };
            if is_open { parens.push(*b); }
            else {
                let paren_type = match b {
                    b')' => b'(',
                    b']' => b'[',
                    b'}' => b'{',
                    _ => { panic!("Nooo"); }
                };
                if parens.len() > 0 && parens[parens.len() - 1] == paren_type {
                    parens.pop();
                } else { // mismatched closing and opening paren type
                    return false;
                }
            }
        }
        return parens.len() == 0;
    }
}
// @lc code=end

