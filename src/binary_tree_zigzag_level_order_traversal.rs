/*
 * @lc app=leetcode id=103 lang=rust
 *
 * [103] Binary Tree Zigzag Level Order Traversal
 */

pub struct Solution;

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

enum Mode {
    LeftRight,
    RightLeft,
}

impl Mode {
    fn switch (&self) -> Mode {
        match self {
            Mode::LeftRight => Mode::RightLeft,
            Mode::RightLeft => Mode::LeftRight,
        }
    }
}

impl Solution {
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        if root.is_none() {
            return vec![];
        }


        let mut parents = vec![root.clone().unwrap()];
        let mut print_order = vec![vec![(*root.unwrap()).borrow().val]];
        let mut cur_mode = Mode::RightLeft;

        while parents.len() > 0 {
            let mut children = vec![];
            for parent in &parents {
                if let Some(x) = &(**parent).borrow().left {
                    children.push(Rc::clone(x));
                }
                if let Some(x) = &(**parent).borrow().right {
                    children.push(Rc::clone(x));
                }
            }
            let iterator: Box<dyn Iterator<Item = &Rc<RefCell<TreeNode>>>> = match cur_mode {
                Mode::LeftRight => Box::new(children.iter()),
                Mode::RightLeft => Box::new(children.iter().rev()),
            };

            let mut vv = vec![];
            vv.reserve(children.len()); 
            for v in iterator.map(|x| (**x).borrow().val) {
                vv.push(v);
            }
            if vv.len() > 0 {
                print_order.push(vv);
            }
            cur_mode = cur_mode.switch();
            parents = children;
        }

        return print_order;
    }
}
// @lc code=end
