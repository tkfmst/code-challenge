/// 572. Subtree of Another Tree
/// Easy
///
/// Given the roots of two binary trees root and subRoot, return true if there is a subtree of root with the same structure and node values of subRoot and false otherwise.
///
/// A subtree of a binary tree tree is a tree that consists of a node in tree and all of this node's descendants. The tree tree could also be considered as a subtree of itself.
///
///
/// Example 1:
/// ![subtree1-tree](./img/subtree1-tree.jpg)
///
/// Input: root = [3,4,5,1,2], subRoot = [4,1,2]
/// Output: true
///
/// Example 2:
/// ![subtree2-tree](./img/subtree2-tree.jpg)
///
/// Input: root = [3,4,5,1,2,null,null,null,null,0], subRoot = [4,1,2]
/// Output: false
///
///
/// Constraints:
/// * The number of nodes in the root tree is in the range [1, 2000].
/// * The number of nodes in the subRoot tree is in the range [1, 1000].
/// * -10^4 <= root.val <= 10^4
// * -10^4 <= subRoot.val <= 10^4
use std::cell::RefCell;
use std::rc::Rc;
use tools::tree_node::TreeNode;

pub struct Solution;
impl Solution {
    pub fn is_subtree(
        root: Option<Rc<RefCell<TreeNode>>>,
        sub_root: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        Self::search(&root, &sub_root)
    }

    fn search(
        root: &Option<Rc<RefCell<TreeNode>>>,
        sub_root: &Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        if root == sub_root {
            return true;
        }

        if let Some(node) = root {
            Self::search(&node.borrow().left, &sub_root)
                || Self::search(&node.borrow().right, &sub_root)
        } else {
            return false;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use tools::tree_node::TreeNode;

    #[test]
    fn test_is_subtree_1() {
        let root = TreeNode::build(vec![Some(3), Some(4), Some(5), Some(1), Some(2)]);
        let sub_root = TreeNode::build(vec![Some(4), Some(1), Some(2)]);
        let output = true;
        assert_eq!(Solution::is_subtree(root, sub_root), output);
    }

    #[test]
    fn test_is_subtree_2() {
        let root = TreeNode::build(vec![
            Some(3),
            Some(4),
            Some(5),
            Some(1),
            Some(2),
            None,
            None,
            None,
            None,
            Some(0),
        ]);
        let sub_root = TreeNode::build(vec![Some(4), Some(1), Some(2)]);
        let output = false;
        assert_eq!(Solution::is_subtree(root, sub_root), output);
    }
}
