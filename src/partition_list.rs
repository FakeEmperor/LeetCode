/*
 * @lc app=leetcode id=86 lang=rust
 *
 * [86] Partition List
 */

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

pub struct Solution {}

// @lc code=start

impl Solution {
    pub fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        let [mut left_head, mut right_head] = [None, None];
        let mut left_tail: &mut Option<Box<ListNode>> = &mut None;
        let mut right_tail: &mut Option<Box<ListNode>>= &mut None;
        
        let mut iter = &head;
        while let Some(node) = iter {
            iter = &node.next;
            let new_val = Box::new(ListNode::new(node.val));


            let mut tl: &mut &mut Option<Box<ListNode>> = &mut &mut None;
            let mut hd: &mut Option<Box<ListNode>> = &mut None;
            [hd, tl] = if node.val < x {
                [&mut left_head, &left_tail]
            } else {
                [&mut right_head, &right_tail]
            };

            if let Some(tail) = tl {
                tail.next = Some(new_val);
                *tl = &mut tail.next;
            } else {
                *hd = Some(new_val);
                *tl = hd;
            }

        }
        if left_head.is_none() { return right_head; }
        else if right_head.is_none() { return left_head; }
        else {
            left_tail.as_mut().unwrap().next = right_head;
            return left_head;
        }
    }
}
// @lc code=end

