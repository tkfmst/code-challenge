/// 144. Binary Tree Preorder Traversal
///
/// Given the root of a binary tree, return the preorder traversal of its nodes' values.
///  
///
/// Example 1:
/// ![inorder_1.jpg](./img/inorder_1.jpg)
///
/// Input: root = [1,null,2,3]
/// Output: [1,2,3]
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
/// Constraints:
///
/// * The number of nodes in the tree is in the range [0, 100].
/// * -100 <= Node.val <= 100
///
/// Follow up: Recursive solution is trivial, could you do it iteratively?
use std::cell::RefCell;
use std::rc::Rc;
use tools::tree_node::TreeNode;

pub struct Solution {}

impl Solution {
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut route = vec![];
        Self::preorder(&root, &mut route);
        route
    }
    fn preorder(root: &Option<Rc<RefCell<TreeNode>>>, route: &mut Vec<i32>) {
        if let Some(node) = root {
            route.push(node.borrow().val);
            match (&node.borrow().left, &node.borrow().right) {
                (Some(_), Some(_)) => {
                    Self::preorder(&node.borrow().left, route);
                    Self::preorder(&node.borrow().right, route);
                }
                (Some(_), None) => Self::preorder(&node.borrow().left, route),
                (None, Some(_)) => Self::preorder(&node.borrow().right, route),
                (None, None) => (),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use tools::tree_node::TreeNode;

    #[test]
    fn test_preorder_traversal_1() {
        let input = TreeNode::build(vec![Some(1), None, Some(2), Some(3)]);
        let output = vec![1, 2, 3];
        assert_eq!(Solution::preorder_traversal(input), output);
    }

    #[test]
    fn test_preorder_traversal_2() {
        let input = TreeNode::build(vec![]);
        let output = vec![];
        assert_eq!(Solution::preorder_traversal(input), output);
    }

    #[test]
    fn test_preorder_traversal_3() {
        let input = TreeNode::build(vec![Some(1)]);
        let output = vec![1];
        assert_eq!(Solution::preorder_traversal(input), output);
    }
}
