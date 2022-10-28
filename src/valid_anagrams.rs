/*
* @lc app=leetcode id=242 lang=rust
*
* [242] Valid Anagram
*/

pub struct Solution {}

// @lc code=start

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut mp = vec![0; 26];
        let bs = s.encode_utf16();
        let bt = t.encode_utf16();
        let a = 'a' as u32 as u16;
        for (chs, cht) in bs.into_iter().zip(bt.into_iter()) {
            mp[(chs - a) as usize] += 1;
            mp[(cht - a) as usize] -= 1;
        }
        for v in mp {
            if v != 0 {
                return false;
            }
        }

        true
    }
}
// @lc code=end
