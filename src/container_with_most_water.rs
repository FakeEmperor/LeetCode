/*
 * @lc app=leetcode id=11 lang=rust
 *
 * [11] Container With Most Water
 */
pub struct Solution;

// @lc code=start
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        if height.len() < 2 { return 0; };
        let mut left_wall_idx = 0;
        let mut right_wall_idx = height.len() -1;
        let mut max = 0;
        while left_wall_idx < right_wall_idx {
            let left = height[left_wall_idx];
            let right = height[right_wall_idx];
            let mut surface = left.min(right) * (right_wall_idx - left_wall_idx) as i32;
            max = max.max(surface);
            if left < right { 
                left_wall_idx += 1;
                while left_wall_idx < right_wall_idx && height[left_wall_idx] <= left {
                    left_wall_idx += 1;
                }
            
            } 
            else { 
                right_wall_idx -= 1;
                while left_wall_idx < right_wall_idx && height[right_wall_idx] <= right {
                    right_wall_idx -= 1;
                }
            };
        }
        return max;
    }
}
// @lc code=end

