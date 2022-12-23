/*
 * @lc app=leetcode id=74 lang=rust
 *
 * [74] Search a 2D Matrix
 */

pub struct Solution {}

// @lc code=start
impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let row_first_vals: Vec<i32> = matrix.iter().map(|row| row[0]).collect();
        let row_idx;
        match row_first_vals.binary_search(&target) {
            Ok(_) => {
                return true;
            }
            Err(row) => {
                if row == 0 {
                    return false;
                } else {
                    row_idx = row - 1;
                }
            }
        }
        match matrix[row_idx].binary_search(&target) {
            Ok(_) => true,
            Err(_) => false,
        }
    }
}
// @lc code=end
