/*
 * @lc app=leetcode id=566 lang=rust
 *
 * [566] Reshape the Matrix
 */

struct Solution {}

// @lc code=start

impl Solution {
    pub fn matrix_reshape(mat: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
        let (n, m, c, r) = (mat.len(), mat[0].len(), c as usize, r as usize);
        if n * m != r * c {
            return mat;
        }

        let mut result = vec![vec![0; c]; r];
        for (idx, value) in mat.into_iter().flatten().enumerate() {
            let cidx = idx % c;
            let ridx = idx / c;
            result[ridx][cidx] = value;
        }
        result
    }
}
// @lc code=end
