/*
 * @lc app=leetcode id=53 lang=rust
 *
 * [53] Maximum Subarray
 */

pub struct Solution {}

// @lc code=start
enum Mode {
    SearchingStart,
    SearchingEnd,
}

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }
        let mut max_sum = nums[0];
        let mut current_sum = 0;
        let mut mode = Mode::SearchingStart;
        for num in nums {
            match mode {
                Mode::SearchingStart => {
                    if num > 0 {
                        current_sum += num;
                        mode = Mode::SearchingEnd;
                    } else if max_sum < 0 {
                        max_sum = max_sum.max(num)
                    }
                }
                Mode::SearchingEnd => {
                    if num < 0 {
                        max_sum = max_sum.max(current_sum);
                        if -num > current_sum {
                            current_sum = 0;
                            mode = Mode::SearchingStart;
                        } else {
                            current_sum += num;
                        }
                    } else {
                        current_sum += num;
                    }
                }
            }
        }
        match mode {
            Mode::SearchingStart => max_sum,
            Mode::SearchingEnd => max_sum.max(current_sum),
        }
    }
}
// @lc code=end
