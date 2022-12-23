/*
 * @lc app=leetcode id=112 lang=rust
 *
 * [112] Path Sum
 */

// Definition for a binary tree node.

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
      right: None
    }
  }
}

pub struct Solution {}


// @lc code=start
use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn _has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        match root {
            Some(root_ref) => {
                let head = root_ref.borrow();
                let nts = target_sum - head.val;
                let (has_left, has_right) = (head.left.is_some(), head.right.is_some());
                // it is leaf - node with no children
                (!has_left && !has_right && nts == 0) ||
                // left is good
                (has_left && Self::_has_path_sum(head.left.clone(), nts)) || 
                // right is good
                (has_right && Self::_has_path_sum(head.right.clone(), nts))
            },
            None => false,
        }
    }
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        Self::_has_path_sum(root, target_sum)
    }
}
// @lc code=end

