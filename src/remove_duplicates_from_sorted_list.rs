/*
 * @lc app=leetcode id=83 lang=rust
 *
 * [83] Remove Duplicates from Sorted List
 */

use std::{borrow::BorrowMut, ops::Deref};

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

struct Solution {}

// @lc code=start

impl Solution {
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut curr = head.as_mut();

        while let Some(curr_value) = curr {
            let mut next = curr_value.next.take();
            while let Some(next_value) = next.as_mut() {
                if curr_value.val == next_value.val {
                    next = next_value.next.take();
                } else {
                    curr_value.next = next;
                    break;
                }
            }
            curr = curr_value.next.as_mut();
        }

        head
    }
}
// @lc code=end

