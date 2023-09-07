/*
 * @lc app=leetcode id=3099 lang=rust
 *
 * [Strobogrammatic Number] Coin Change
 */




pub struct Solution;

// @lc code=start
impl Solution {
    pub fn is_strobogrammatic(num: String) -> bool {
        let s = num.as_bytes();
        for (idx, n) in s.iter().enumerate() {
            if idx >= (num.len() + 1) / 2 { break; }
            let mirror = match n {
                b'9' => b'6',
                b'6' => b'9',
                b'8' => b'8',
                b'1' => b'1',
                b'0' => b'0',
                _ => { return false; }
            };
            if s[s.len() - 1 - idx] != mirror { return false; }
        }
        return true
    }
}
// @lc code=end

