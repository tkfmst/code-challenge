/// 110. Balanced Binary Tree
///
/// Given a binary tree, determine if it is hihgt-balanced.
///
/// For this problem, a hihgt-balanced binary tree is defined as:
///
/// > a binary tree in which the left and right subtrees of every node differ in hihgt by no more than 1.
///
/// Example 1:
/// ![balance_1.jpg](./img/balance_1.jpg)
///
/// Input: root = [3,9,20,null,null,15,7]
/// Output: true
///
/// Example 2:
/// ![balance_2.jpg](./img/balance_2.jpg)
/// Input: root = [1,2,2,3,3,null,null,4,4]
/// Output: false
///
/// Example 3:
///
/// Input: root = []
/// Output: true
///
///
/// Constraints:
/// - The number of nodes in the tree is in the range [0, 5000].
/// - -104 <= Node.val <= 104
use std::cell::RefCell;
use std::rc::Rc;
use tools::tree_node::TreeNode;

pub struct Solution {}

impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(node) = root {
            (Self::hight(&node.borrow().left, 0) - Self::hight(&node.borrow().right, 0)).abs() <= 1
                // && Self::is_balanced(&node.borrow().left)
                && Self::is_balanced(node.borrow().left.as_ref().map(|x| Rc::clone(&x)))
                && Self::is_balanced(node.borrow().right.as_ref().map(|x| Rc::clone(&x)))
        } else {
            true
        }
    }

    fn hight(root: &Option<Rc<RefCell<TreeNode>>>, h: i32) -> i32 {
        if let Some(node) = root {
            Self::hight(&node.borrow().left, h + 1).max(Self::hight(&node.borrow().right, h + 1))
        } else {
            h
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use tools::tree_node::TreeNode;

    #[test]
    fn test_is_balanced1() {
        let root = TreeNode::build(vec![
            Some(3),
            Some(9),
            Some(20),
            None,
            None,
            Some(15),
            Some(7),
        ]);
        assert_eq!(Solution::is_balanced(root), true);
    }

    #[test]
    fn test_is_balanced2() {
        let root = TreeNode::build(vec![
            Some(1),
            Some(2),
            Some(2),
            Some(3),
            Some(3),
            None,
            None,
            Some(4),
            Some(4),
        ]);
        assert_eq!(Solution::is_balanced(root), false);
    }

    #[test]
    fn test_is_balanced3() {
        let root = TreeNode::build(vec![]);
        assert_eq!(Solution::is_balanced(root), true);
    }

    #[test]
    fn test_is_balanced4() {
        let root = TreeNode::build(vec![
            Some(1),
            Some(2),
            Some(2),
            Some(3),
            None,
            None,
            Some(3),
            Some(4),
            None,
            None,
            Some(4),
        ]);
        assert_eq!(Solution::is_balanced(root), false);
    }
}
