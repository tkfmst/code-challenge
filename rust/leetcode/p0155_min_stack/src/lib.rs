/// 155. Min Stack
///
/// Design a stack that supports push, pop, top, and retrieving the minimum element in constant time.
///
/// Implement the MinStack class:
/// * MinStack() initializes the stack object.
/// * void push(int val) pushes the element val onto the stack.
/// * void pop() removes the element on the top of the stack.
/// * int top() gets the top element of the stack.
/// * int getMin() retrieves the minimum element in the stack.
///
///
///
/// Example 1:
///
/// Input
/// ["MinStack","push","push","push","getMin","pop","top","getMin"]
/// [[],[-2],[0],[-3],[],[],[],[]]
///
/// Output
/// [null,null,null,null,-3,null,0,-2]
///
/// Explanation
/// MinStack minStack = new MinStack();
/// minStack.push(-2);
/// minStack.push(0);
/// minStack.push(-3);
/// minStack.getMin(); // return -3
/// minStack.pop();
/// minStack.top();    // return 0
/// minStack.getMin(); // return -2
///
/// Constraints:
/// * -231 <= val <= 231 - 1
/// * Methods pop, top and getMin operations will always be called on non-empty stacks.
/// * At most 3 * 104 calls will be made to push, pop, top, and getMin.
type Min = i32;

pub struct MinStack {
    stack: Vec<(Min, i32)>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    pub fn new() -> Self {
        MinStack { stack: vec![] }
    }

    pub fn push(&mut self, val: i32) {
        let mut min = val;

        if let Some(&(m, _)) = self.stack.last() {
            min = min.min(m);
        }

        self.stack.push((min, val));
    }

    pub fn pop(&mut self) {
        self.stack.pop();
    }

    pub fn top(&self) -> i32 {
        self.stack.last().unwrap().1
    }

    pub fn get_min(&self) -> i32 {
        self.stack.last().unwrap().0
    }
}

#[cfg(test)]
mod tests {
    use super::MinStack;
    #[test]
    fn test_min_stack_1() {
        let mut obj = MinStack::new();
        obj.push(-2);
        obj.push(0);
        obj.push(-3);
        assert_eq!(obj.get_min(), -3);

        obj.pop();
        assert_eq!(obj.top(), 0);

        assert_eq!(obj.get_min(), -2);
    }
}
