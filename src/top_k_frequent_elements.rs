/*
 * @lc app=leetcode id=347 lang=rust
 *
 * [347] Top K Frequent Elements
 */


pub struct Solution {}

// @lc code=start
use std::collections::HashMap;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut answer = vec![];
        
        let mut frequencies = HashMap::<i32, usize>::new();
        let mut numbers = HashMap::<usize, Vec::<i32>>::new();
        
        for num in nums {
            frequencies.entry(num).and_modify(|f| *f+=1).or_insert(1);
        }

        let mut max = 0;
        for (number, freq) in frequencies.iter() {
            max = max.max(*freq);
            numbers.entry(*freq).and_modify(|v| v.push(*number)).or_insert(vec![*number]);
        }

        let mut left = nums.len();
        let mut check = max + 1;

        while left > 0 && answer.len() < k as usize {
            check = left.min(check - 1);
            if let Some(v) = numbers.get(&check) {
                for x in v {
                    if answer.len() >= k as usize { break; }
                    answer.push(*x);
                }
                left -= check * v.len();
            }
        }
        

        return answer;
    }
}
// @lc code=end

