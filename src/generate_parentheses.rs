/*
 * @lc app=leetcode id=22 lang=rust
 *
 * [22] Generate Parentheses
 */

pub struct Solution;

// @lc code=start

fn generate_parens(prefix: String, left_balance: usize, right_balance: usize, vec: &mut Vec<String>) {
    // println!("Hello, n: {}, prefix: {}, imbalance: {}\n", left_parens, prefix, to_close);
    if left_balance == 0 && right_balance == 0 { vec.push(prefix); return; }
    if left_balance > 0 {
        generate_parens(prefix.clone() + "(", left_balance - 1, right_balance, vec)
    }
    if right_balance > 0 && right_balance > left_balance {
        generate_parens(prefix.clone() + ")", left_balance, right_balance - 1, vec)
    }
}

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        if n == 1 { return vec!["()".to_owned()]; }
        let mut variants = vec![];
        generate_parens("".to_owned(), n as usize, n as usize, &mut variants);
        return variants;
    }
}
// @lc code=end

// (), ((), ((()
// ()(), ()(() |  (()), (()()  | ((()))
// ()()(), ()(()) | (())() (()())