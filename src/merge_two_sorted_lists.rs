/*
 * @lc app=leetcode id=21 lang=rust
 *
 * [21] Merge Two Sorted Lists
 */

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

pub struct Solution {

}

// @lc code=start

impl Solution {

    pub fn merge_two_lists(mut list1: Option<Box<ListNode>>, mut list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut prehead = ListNode::new(0);
        let mut cur = &mut prehead;
        while let (Some(elem1), Some(elem2)) = (&list1, &list2) {
            if elem1.val < elem2.val {
                cur.next = list1.take();
                cur = cur.next.as_mut().unwrap();
                list1 = cur.next.take();
              } else {
                  cur.next = list2.take();
                cur = cur.next.as_mut().unwrap();
                list2 = cur.next.take();
            }
        }
        
        cur.next = list1.or(list2);
        return prehead.next;
    }
}
// @lc code=end

