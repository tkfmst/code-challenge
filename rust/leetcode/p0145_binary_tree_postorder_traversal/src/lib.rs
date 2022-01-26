/// 145. Binary Tree Postorder Traversal
///
/// Given the root of a binary tree, return the postorder traversal of its nodes' values.
///
/// Example 1:
/// ![pre1.jpg](./img/pre1.jpg)
///
/// Input: root = [1,null,2,3]
/// Output: [3,2,1]
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
/// * The number of the nodes in the tree is in the range [0, 100].
/// * -100 <= Node.val <= 100
use std::cell::RefCell;
use std::rc::Rc;
use tools::tree_node::TreeNode;

pub struct Solution {}

impl Solution {
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        Self::traverse(&root)
    }

    pub fn traverse(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut route: Vec<i32> = vec![];
        if let Some(node) = root {
            match (&node.borrow().left, &node.borrow().right) {
                (Some(_), Some(_)) => {
                    route.append(Self::traverse(&node.borrow().left).as_mut());
                    route.append(Self::traverse(&node.borrow().right).as_mut());
                    route.push(node.borrow().val);
                    route
                }
                (Some(_), None) => {
                    route.append(Self::traverse(&node.borrow().left).as_mut());
                    route.push(node.borrow().val);
                    route
                }
                (None, Some(_)) => {
                    route.append(Self::traverse(&node.borrow().right).as_mut());
                    route.push(node.borrow().val);
                    route
                }
                (None, None) => {
                    route.push(node.borrow().val);
                    route
                }
            }
        } else {
            route
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use tools::tree_node::TreeNode;

    #[test]
    fn test_postorder_traversal_1() {
        let input = TreeNode::build(vec![Some(1), None, Some(2), Some(3)]);
        let output = vec![3, 2, 1];
        assert_eq!(Solution::postorder_traversal(input), output);
    }

    #[test]
    fn test_postorder_traversal_2() {
        let input = TreeNode::build(vec![]);
        let output = vec![];
        assert_eq!(Solution::postorder_traversal(input), output);
    }

    #[test]
    fn test_postorder_traversal_3() {
        let input = TreeNode::build(vec![Some(1)]);
        let output = vec![1];
        assert_eq!(Solution::postorder_traversal(input), output);
    }
}
