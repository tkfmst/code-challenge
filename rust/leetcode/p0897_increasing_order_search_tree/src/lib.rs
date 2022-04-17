/// 897. Increasing Order Search Tree
/// Easy
///
/// Given the root of a binary search tree, rearrange the tree in in-order so that the leftmost node in the tree is now the root of the tree, and every node has no left child and only one right child.
///
///
/// Example 1:
///
/// Input: root = [5,3,6,2,4,null,8,1,null,null,null,7,9]
/// Output: [1,null,2,null,3,null,4,null,5,null,6,null,7,null,8,null,9]
///
/// Example 2:
///
/// Input: root = [5,1,7]
/// Output: [1,null,5,null,7]
///
///
/// Constraints:
/// * The number of nodes in the given tree will be in the range [1, 100].
/// * 0 <= Node.val <= 1000
use std::cell::RefCell;
use std::rc::Rc;
use tools::tree_node::TreeNode;

pub struct Solution {}
impl Solution {
    pub fn increasing_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        fn inorder(node: &Option<Rc<RefCell<TreeNode>>>, stack: &mut Vec<i32>) {
            if let Some(cur_node) = node {
                let cur_node = cur_node.borrow();
                inorder(&cur_node.left, stack);
                stack.push(cur_node.val);
                inorder(&cur_node.right, stack);
            }
        }
        let mut stack = vec![];
        inorder(&root, &mut stack);
        stack.iter().rev().fold(None, |right, &val| {
            Some(Rc::new(RefCell::new(TreeNode {
                val,
                right,
                left: None,
            })))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use tools::tree_node::TreeNode;

    #[test]
    fn test_increasing_bst_1() {
        let root = TreeNode::build(vec![
            Some(5),
            Some(3),
            Some(6),
            Some(2),
            Some(4),
            None,
            Some(8),
            Some(1),
            None,
            None,
            None,
            Some(7),
            Some(9),
        ]);
        let output = TreeNode::build(vec![
            Some(1),
            None,
            Some(2),
            None,
            Some(3),
            None,
            Some(4),
            None,
            Some(5),
            None,
            Some(6),
            None,
            Some(7),
            None,
            Some(8),
            None,
            Some(9),
        ]);

        assert_eq!(Solution::increasing_bst(root), output);
    }

    #[test]
    fn test_increasing_bst_2() {
        let root = TreeNode::build(vec![Some(5), Some(1), Some(7)]);
        let output = TreeNode::build(vec![Some(1), None, Some(5), None, Some(7)]);
        assert_eq!(Solution::increasing_bst(root), output);
    }
}
