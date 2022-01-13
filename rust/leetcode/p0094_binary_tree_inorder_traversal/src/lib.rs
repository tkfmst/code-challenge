/// 94. Binary Tree Inorder Traversal
///
/// Given the root of a binary tree, return the inorder traversal of its nodes' values.
///
/// Example 1:
///
/// Input: root = [1,null,2,3]
/// Output: [1,3,2]
///
/// Example 2:
///
/// Input: root = []
/// Output: []
///
/// Example 3:
///
/// Input: root = [1]
/// Output: [1]
///
///  
///
/// Constraints:
///
/// * The number of nodes in the tree is in the range [0, 100].
/// * -100 <= Node.val <= 100
use std::cell::RefCell;
use std::rc::Rc;

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
            right: None,
        }
    }
}
pub struct Solution;

impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut order = vec![];
        traverse(&mut order, root);
        order
    }
}

fn traverse(order: &mut Vec<i32>, root: Option<Rc<RefCell<TreeNode>>>) {
    if let Some(n) = root {
        let n = n.borrow();
        traverse(order, n.left.as_ref().map(|x| Rc::clone(&x)));
        order.push(n.val);
        traverse(order, n.right.as_ref().map(|x| Rc::clone(&x)));
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use super::TreeNode;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn test_inorder_traversal1() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: None,
                }))),
                right: None,
            }))),
        })));
        assert_eq!(Solution::inorder_traversal(root), vec![1, 3, 2]);
    }

    #[test]
    fn test_inorder_traversal2() {
        let root: Option<Rc<RefCell<TreeNode>>> = None;
        assert_eq!(Solution::inorder_traversal(root), vec![]);
    }

    #[test]
    fn test_inorder_traversal3() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: None,
        })));
        assert_eq!(Solution::inorder_traversal(root), vec![1]);
    }
}
