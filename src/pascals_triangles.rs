/*
 * @lc app=leetcode id=118 lang=rust
 *
 * [118] Pascal's Triangle
 */


pub struct Solution {}

// @lc code=start


impl Solution {    
    pub fn pascal(result: &mut Vec<Vec<i32>>, rows: usize) {
        static INITIAL: &[&[i32]] = &[&[1], &[1, 1], &[1, 2, 1], &[1, 3, 3, 1]]; 
        if rows > 4 {
            Self::pascal(result, rows-1);
            let mut new_row: Vec<i32> = vec![1; rows];
            for (idx, w) in result[result.len()-1].windows(2).map(|w| w.iter().sum()).enumerate() {
                new_row[idx+1] = w;
            }
            result.push(new_row);
            return;
        }
        if rows <= 4 {
            for i in 0..rows {
                result.push(Vec::from(INITIAL[i]))
            }
        }

    }
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let num_rows = num_rows as usize;
        let mut result = Vec::with_capacity(num_rows);
        Self::pascal(&mut result, num_rows);        
        result
    }
}
// @lc code=end
