/*
 * @lc app=leetcode id=232 lang=rust
 *
 * [232] Implement Queue using Stacks
 */


// @lc code=start
use std::collections::{VecDeque};

#[derive(Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct MyQueue {
    buf: VecDeque<i32>,
    queue: VecDeque<i32>
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyQueue {

    
    /**
     * Your MyQueue object will be instantiated and called as such:
     * let obj = MyQueue::new();
     * obj.push(x);
     * let ret_2: i32 = obj.pop();
     * let ret_3: i32 = obj.peek();
     * let ret_4: bool = obj.empty();
     */
    fn new() -> Self {
        MyQueue { buf: VecDeque::<_>::with_capacity(10), queue: VecDeque::<_>::with_capacity(10) }
    }
    
    fn push(&mut self, x: i32) {
        self.buf.push_back(x);
    }

    fn _populate_queue(&mut self) {
        if self.queue.is_empty() {
            while let Some(value) = self.buf.pop_back() {
                self.queue.push_back(value)
            }
        }
        
    }
    
    fn pop(&mut self) -> i32 {
        self._populate_queue();
        self.queue.pop_back().unwrap()
    }
    
    fn peek(&mut self) -> i32 {
        self._populate_queue();
        *self.queue.back().unwrap()
    }
    
    fn empty(&self) -> bool {
        self.buf.is_empty() && self.queue.is_empty()
    }
}

// @lc code=end

