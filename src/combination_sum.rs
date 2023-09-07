/*
 * @lc app=leetcode id=39 lang=rust
 *
 * [39] Combination Sum
 */


pub struct Solution;

// @lc code=start

impl Solution {
    pub fn solve_combinations_for(skip: usize, target: i32, candidates: &Vec<i32>, solutions: &mut Vec<Vec<i32>>, intermediate: &Vec<i32>) {
        for (idx, candidate) in candidates.iter().skip(skip).enumerate() {
            if *candidate > target { break; }
            let mut solution = intermediate.clone();
            solution.push(*candidate);
            if *candidate == target {
                solutions.push(solution);
            } else {
                Self::solve_combinations_for(idx + skip, target - candidate, candidates, solutions, &solution);
            }       
        }
    }

    pub fn combination_sum(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        if target == 0 { return vec![]; }
        candidates.sort_unstable();
        let mut solutions = vec![];
        Self::solve_combinations_for(0, target, &candidates, &mut solutions, &vec![]);
        solutions
    }
}
// @lc code=end

