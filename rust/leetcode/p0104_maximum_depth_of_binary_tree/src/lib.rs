/// 104. Maximum Depth of Binary Tree
/// Easy
///
/// Given the root of a binary tree, return its maximum depth.
///
/// A binary tree's maximum depth is the number of nodes along the longest path from the root node down to the farthest leaf node.
///
///
///
/// Example 1:
/// ![tmp-tree.jpg](./img/tmp-tree.jpg)
///
/// Input: root = [3,9,20,null,null,15,7]
/// Output: 3
///
/// Example 2:
///
/// Input: root = [1,null,2]
/// Output: 2
///
///
///
/// Constraints:
/// - The number of nodes in the tree is in the range [0, 104].
/// - -100 <= Node.val <= 100
use std::cell::RefCell;
use std::rc::Rc;

use tools::tree_node::TreeNode;

pub struct Solution {}

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::depth(0, &root)
    }

    fn depth(d: i32, root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = root {
            Self::depth(d + 1, &node.borrow().left).max(Self::depth(d + 1, &node.borrow().right))
        } else {
            d
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use tools::tree_node::TreeNode;

    #[test]
    fn test_max_depth1() {
        let root = TreeNode::build(vec![
            Some(3),
            Some(9),
            Some(20),
            None,
            None,
            Some(15),
            Some(7),
        ]);
        assert_eq!(Solution::max_depth(root), 3);
    }

    #[test]
    fn test_max_depth2() {
        let root = TreeNode::build(vec![Some(1), None, Some(2)]);
        assert_eq!(Solution::max_depth(root), 2);
    }
}
