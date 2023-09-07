/*
 * @lc app=leetcode id=48 lang=rust
 *
 * [48] Rotate Image
 */

pub struct Solution;

// @lc code=start
impl Solution {
    pub fn rotate_outer(skip: usize, size: usize, matrix: &mut Vec<Vec<i32>>) {
        if size <= 1 { return; }
        // remember all sides
        let t_00 = matrix[skip].clone();
        let t_01 = matrix[skip..skip + size].iter().rev().map(|v| v[skip + size - 1]).collect::<Vec<i32>>();
        let t_10 = matrix[skip + size - 1].clone();
        let t_11 = matrix[skip..skip + size].iter().rev().map(|v| v[skip]).collect::<Vec<i32>>();

        t_00.iter().enumerate().map(|(i, v)| matrix[skip + i][skip + size - 1] = *v).count();
        t_01.iter().enumerate().map(|(i, v)| matrix[skip + size - 1][skip + i] = *v).count();
        t_10.iter().enumerate().map(|(i, v)| matrix[skip + i][skip] = *v).count();
        t_11.iter().enumerate().map(|(i, v)| matrix[skip][skip + i] = *v).count();
        //matrix[skip + size - 1] = t_01; 
        //matrix[skip..skip + size].iter_mut().enumerate().map(|(i, v)| v[skip] = t_10[i]);

        // otherwise it's a center element which we don't need to rotate
        Self::rotate_outer(skip + 1, size - 2, matrix);
    }


    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        Self::rotate_outer(0, matrix.len(), matrix);
    }
}
// @lc code=end
