/// 617. Merge Two Binary Trees
/// Easy
///
/// You are given two binary trees root1 and root2.
///
/// Imagine that when you put one of them to cover the other, some nodes of the two trees are overlapped while the others are not. You need to merge the two trees into a new binary tree. The merge rule is that if two nodes overlap, then sum node values up as the new value of the merged node. Otherwise, the NOT null node will be used as the node of the new tree.
///
/// Return the merged tree.
///
/// Note: The merging process must start from the root nodes of both trees.
///
/// Example 1:
/// ![merge.jpg](./img/merge.jpg)
///
/// Input: root1 = [1,3,2,5], root2 = [2,1,3,null,4,null,7]
/// Output: [3,4,5,5,4,null,7]
///
/// Example 2:
///
/// Input: root1 = [1], root2 = [1,2]
/// Output: [2,2]
///
///
/// Constraints:
/// * The number of nodes in both trees is in the range [0, 2000].
/// * -10^4 <= Node.val <= 10^4
use std::cell::RefCell;
use std::rc::Rc;
use tools::tree_node::TreeNode;

pub struct Solution {}
impl Solution {
    pub fn merge_trees(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        Self::merge(&root1, &root2)
    }

    fn merge(
        root1: &Option<Rc<RefCell<TreeNode>>>,
        root2: &Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match (root1, root2) {
            (Some(n1), Some(n2)) => Some(Rc::new(RefCell::new(TreeNode {
                val: n1.borrow().val + n2.borrow().val,
                left: Self::merge(&n1.borrow().left, &n2.borrow().left),
                right: Self::merge(&n1.borrow().right, &n2.borrow().right),
            }))),
            (Some(n1), None) => root1.clone(),
            (None, Some(n2)) => root2.clone(),
            (None, None) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use tools::tree_node::TreeNode;

    #[test]
    fn super_merge_trees_1() {
        let root1 = TreeNode::build(vec![Some(1), Some(3), Some(2), Some(5)]);
        let root2 = TreeNode::build(vec![
            Some(2),
            Some(1),
            Some(3),
            None,
            Some(4),
            None,
            Some(7),
        ]);
        let output = TreeNode::build(vec![
            Some(3),
            Some(4),
            Some(5),
            Some(5),
            Some(4),
            None,
            Some(7),
        ]);
        assert_eq!(Solution::merge_trees(root1, root2), output);
    }

    #[test]
    fn super_merge_trees_2() {
        let root1 = TreeNode::build(vec![Some(1)]);
        let root2 = TreeNode::build(vec![Some(1), Some(2)]);
        let output = TreeNode::build(vec![Some(2), Some(2)]);
        assert_eq!(Solution::merge_trees(root1, root2), output);
    }
}
