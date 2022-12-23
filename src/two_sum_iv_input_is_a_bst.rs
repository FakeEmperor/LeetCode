/*
 * @lc app=leetcode id=653 lang=rust
 *
 * [653] Two Sum IV - Input is a BST
 */

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

struct Solution {}

// @lc code=start
// Definition for a binary tree node.



use std::rc::Rc;
use std::cell::{RefCell, Ref};
impl Solution {
    pub fn find_target(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> bool {
        match root {
            Some(node) => Self::_iterate_over_bst(&node.borrow(), &node.borrow(), k),
            None => false
        }
    }
    fn _iterate_over_bst(current_node: &Ref<'_, TreeNode>, root: &Ref<'_, TreeNode>, k: i32) -> bool {
        let to_find = k - current_node.val;
        // search in left arm of node
        if to_find != current_node.val && Self::_find_value(root, to_find) {
            return true;
        }
        
        // couldn't find to_find -> iterate next
        match &current_node.left {
            Some(left_value) => {
                if Self::_iterate_over_bst(&left_value.borrow(), root, k) {return true;};
            },
            None => {}
        };
        match &current_node.right {
            Some(right_value) => {
                if Self::_iterate_over_bst(&right_value.borrow(), root, k) {return true;};
            },
            None => {}
        }
        false
    }

    fn _find_value(node: &Ref<'_, TreeNode>, a: i32) -> bool {
        if node.val == a { return true; }
        let search_node = {
            if a < node.val { node.left.as_ref() }
            else { node.right.as_ref() }
        };

        match search_node {
            Some(search_node_) => {
                Self::_find_value(&search_node_.borrow(), a)
            },
            None => false
        }
    }
}
// @lc code=end

