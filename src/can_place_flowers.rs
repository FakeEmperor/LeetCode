/*
 * @lc app=leetcode id=605 lang=rust
 *
 * [605] Can Place Flowers
 */

pub struct Solution;

// @lc code=start
impl Solution {
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        if n == 0 { return true; }
        if flowerbed.len() == 1 { return flowerbed[0] == 0; }

        let mut allowed_flowers = 0;

        let mut spaces = 0;
        let mut was_flower = false;
        let mut was_place = false;
        for (idx, slot) in flowerbed.iter().enumerate() {
            // println!("Current slot: [{}]={}", idx, *slot);
            if *slot == 1 {
                if was_place { spaces -= 1; }
                was_flower = true;
                was_place = false;

                allowed_flowers += (spaces + 1) / 2;
                spaces = 0;

                if allowed_flowers >= n { return true; }
            } else {
                if !was_flower { spaces += 1; }

                was_flower = false;
                was_place = true;
            }
            // println!("Spaces: {}", spaces);
            
        }
        // println!("Ultimate Spaces: {}", spaces);
        allowed_flowers += (spaces + 1) / 2;
        return allowed_flowers >= n;
    }
}
// @lc code=end

