/*
 * @lc app=leetcode id=12 lang=rust
 *
 * [12] Integer to Roman
 */

pub struct Solution {}

// @lc code=start
impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let powers = vec![1, 5, 10, 50, 100, 500, 1000];
        let chars = vec!['I', 'V', 'X', 'L', 'C', 'D', 'M'];
        


        let mut leftovers = num;
        let mut romans: Vec<char> = vec![];
        let mut power_idx = powers.len()-1;
        let mut current_power = powers[power_idx];
        while leftovers > 0 {
            let mut diff = leftovers - current_power;
            if diff > 0 {
                leftovers = diff;
                romans.push(chars[power_idx]);
            } else {
                // search for appropriate power
                let mut balance_diff = diff;
                let mut next_power_idx = 0;
                for (npidx, np) in powers.iter().enumerate().rev().skip(powers.len() - power_idx) {
                    
                }
            }
        }

        return "".to_string();
    }
}
// @lc code=end

