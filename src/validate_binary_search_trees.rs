/*
 * @lc app=leetcode id=98 lang=rust
 *
 * [98] Validate Binary Search Tree
 */

pub struct Solution {}
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

// @lc code=start
// Definition for a binary tree node.

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn _is_valid_bst(root: &Rc<RefCell<TreeNode>>, lower: i64, upper: i64) -> bool {
        let head = root.borrow();
        let head_val = head.val as i64;

        head_val < upper
            && head_val > lower
            && match head.left.as_ref() {
                Some(left_arm) => Self::_is_valid_bst(left_arm, lower, head_val as i64),
                None => true,
            }
            && match head.right.as_ref() {
                Some(right_arm) => Self::_is_valid_bst(&right_arm, head_val as i64, upper),
                None => true,
            }
    }

    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match root {
            Some(head) => Self::_is_valid_bst(&head, i64::min_value(), i64::max_value()),
            None => true,
        }
    }
}
// @lc code=end
