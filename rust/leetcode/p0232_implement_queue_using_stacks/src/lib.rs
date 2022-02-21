/// 232. Implement Queue using Stacks
/// Easy
///
/// Implement a first in first out (FIFO) queue using only two stacks. The implemented queue should support all the functions of a normal queue (push, peek, pop, and empty).
///
/// Implement the MyQueue class:
///
/// void push(int x) Pushes element x to the back of the queue.
/// int pop() Removes the element from the front of the queue and returns it.
/// int peek() Returns the element at the front of the queue.
/// boolean empty() Returns true if the queue is empty, false otherwise.
///
/// Notes:
///
/// You must use only standard operations of a stack, which means only push to top, peek/pop from top, size, and is empty operations are valid.
/// Depending on your language, the stack may not be supported natively. You may simulate a stack using a list or deque (double-ended queue) as long as you use only a stack's standard operations.
///
/// Example 1:
///
/// Input
/// ["MyQueue", "push", "push", "peek", "pop", "empty"]
/// [[], [1], [2], [], [], []]
/// Output
/// [null, null, null, 1, 1, false]
///
/// Explanation
/// MyQueue myQueue = new MyQueue();
/// myQueue.push(1); // queue is: [1]
/// myQueue.push(2); // queue is: [1, 2] (leftmost is front of the queue)
/// myQueue.peek(); // return 1
/// myQueue.pop(); // return 1, queue is [2]
/// myQueue.empty(); // return false
///
///
///
/// Constraints:
///
/// * 1 <= x <= 9
/// * At most 100 calls will be made to push, pop, peek, and empty.
/// * All the calls to pop and peek are valid.
///
/// Follow-up: Can you implement the queue such that each operation is amortized O(1) time complexity? In other words, performing n operations will take overall O(n) time even if one of those operations may take longer.

pub struct MyQueue {
    s1: Vec<i32>,
    s2: Vec<i32>,
    front: i32,
}

impl MyQueue {
    pub fn new() -> Self {
        Self {
            s1: vec![],
            s2: vec![],
            front: 0,
        }
    }

    pub fn push(&mut self, x: i32) {
        self.s1.push(x);
        if self.s1.len() == 1 {
            self.front = x;
        }
    }

    pub fn pop(&mut self) -> i32 {
        while self.s1.len() > 0 {
            let v = self.s1.pop().unwrap();
            if self.s1.len() == 1 {
                self.front = v;
            }
            self.s2.push(v);
        }

        let v = self.s2.pop().unwrap();

        while self.s2.len() > 0 {
            self.s1.push(self.s2.pop().unwrap());
        }

        v
    }

    pub fn peek(&self) -> i32 {
        self.front
    }

    pub fn empty(&self) -> bool {
        self.s1.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::MyQueue;

    #[test]
    fn test_my_queue() {
        let mut q = MyQueue::new();
        q.push(1);
        q.push(2);
        assert_eq!(q.peek(), 1);
        assert_eq!(q.pop(), 1);
        assert_eq!(q.empty(), false);
    }
}
