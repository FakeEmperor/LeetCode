/*
 * @lc app=leetcode id=36 lang=rust
 *
 * [36] Valid Sudoku
 */


pub struct Solution {}

// @lc code=start
use std::{collections::HashSet};


impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut row_checks: Vec<HashSet<char>> = (0..9).map(|_|{HashSet::<char>::with_capacity(9)}).collect();
        let mut column_checks: Vec<HashSet<char>> = (0..9).map(|_|{HashSet::<char>::with_capacity(9)}).collect();
        let mut boxes: Vec<HashSet<char>> = (0..9).map(|_|{HashSet::<char>::with_capacity(9)}).collect();
        //
        // 0 0 0 | 1 1 1 | 2 2 2 |
        // 0 0 0 | 1 1 1 | 2 2 2 |
        // 0 0 0 | 1 1 1 | 2 2 2 |
        // ----- | ----- | ----- |
        // 3 3 3 | 4 4 4 | 5 5 5 |
        // 3 3 3 | 4 4 4 | 5 5 5 |
        // 3 3 3 | 4 4 4 | 5 5 5 |
        // ----- | ----- | ----- |
        // 6 6 6 | 7 7 7 | 8 8 8 |
        // 6 6 6 | 7 7 7 | 8 8 8 |
        // 6 6 6 | 7 7 7 | 8 8 8 |

        for (i, row) in board.into_iter().enumerate() {
            for (j, ch) in row.into_iter().enumerate() {
                if ch == '.' { continue;}
                if row_checks[i].contains(&ch) {/*println!("Failed in row checks at {},{} = {}", i, j, ch);*/ return false;}
                row_checks[i].insert(ch);
                if column_checks[j].contains(&ch) {/*println!("Failed in column checks at {},{} = {}", i, j, ch);*/ return false;}
                column_checks[j].insert(ch);
                // get box id
                let box_id = 3 * (i / 3) + j / 3;
                if boxes[box_id].contains(&ch) {/*println!("Failed in box checks at {},{}[box_id={}] = {}", i, j, box_id, ch);*/ return false;}
                boxes[box_id].insert(ch);
            }
        }
        true
    }
}
// @lc code=end

