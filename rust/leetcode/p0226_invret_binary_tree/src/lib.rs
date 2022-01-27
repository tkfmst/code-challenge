/// 226. Invert Binary Tree
///
/// Given the root of a binary tree, invert the tree, and return its root.
///
/// Example 1:
/// ![invert1-tree.jpg](./img/invert1-tree.jpg)
///
/// Input: root = [4,2,7,1,3,6,9]
/// Output: [4,7,2,9,6,3,1]
///
/// Example 2:
/// ![invert2-tree.jpg](./img/invert2-tree.jpg)
///
/// Input: root = [2,1,3]
/// Output: [2,3,1]
///
/// Example 3:
///
/// Input: root = []
/// Output: []
///
///  
/// Constraints:
/// * The number of nodes in the tree is in the range [0, 100].
/// * -100 <= Node.val <= 100
use std::cell::RefCell;
use std::rc::Rc;
use tools::tree_node::TreeNode;

pub struct Solution {}

impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::invert(&root)
    }
    fn invert(root: &Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root {
            let mut bnode = node.borrow_mut();
            let old_left = bnode.left.take();
            let old_right = bnode.right.take();
            bnode.left = Self::invert(&old_right);
            bnode.right = Self::invert(&old_left);
            Some(Rc::clone(node))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use tools::tree_node::TreeNode;

    #[test]
    fn test_invert_tree_1() {
        let input = TreeNode::build(vec![
            Some(4),
            Some(2),
            Some(7),
            Some(1),
            Some(3),
            Some(6),
            Some(9),
        ]);
        let output = TreeNode::build(vec![
            Some(4),
            Some(7),
            Some(2),
            Some(9),
            Some(6),
            Some(3),
            Some(1),
        ]);
        assert_eq!(Solution::invert_tree(input), output);
    }

    #[test]
    fn test_invert_tree_2() {
        let input = TreeNode::build(vec![Some(2), Some(1), Some(3)]);
        let output = TreeNode::build(vec![Some(2), Some(3), Some(1)]);
        assert_eq!(Solution::invert_tree(input), output);
    }

    #[test]
    fn test_invert_tree_3() {
        let input = TreeNode::build(vec![]);
        let output = TreeNode::build(vec![]);
        assert_eq!(Solution::invert_tree(input), output);
    }
}
