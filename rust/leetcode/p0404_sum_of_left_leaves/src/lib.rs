/// 404. Sum of Left Leaves
/// Easy
///
/// Given the root of a binary tree, return the sum of all left leaves.
///
/// A leaf is a node with no children. A left leaf is a leaf that is the left child of another node.
///
/// Example 1:
/// ![leftsum-tree](./img/leftsum-tree.jpg)
///
/// Input: root = [3,9,20,null,null,15,7]
/// Output: 24
/// Explanation: There are two left leaves in the binary tree, with values 9 and 15 respectively.
///
/// Example 2:
///
/// Input: root = [1]
/// Output: 0
/// Constraints:
///
/// * The number of nodes in the tree is in the range [1, 1000].
/// * -1000 <= Node.val <= 1000
use std::cell::RefCell;
use std::rc::Rc;
use tools::tree_node::TreeNode;

pub struct Solution {}
impl Solution {
    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::helper(&root)
    }
    fn helper(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = root {
            if let Some(l) = &node.borrow().left {
                if let (None, None) = (&l.borrow().left, &l.borrow().right) {
                    return l.borrow().val + Self::helper(&node.borrow().right);
                }
            }

            Self::helper(&node.borrow().left) + Self::helper(&node.borrow().right)
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use tools::tree_node::TreeNode;

    #[test]
    fn test_sum_of_left_leaves_1() {
        let root = TreeNode::build(vec![
            Some(3),
            Some(9),
            Some(20),
            None,
            None,
            Some(15),
            Some(7),
        ]);
        let output = 24;
        assert_eq!(Solution::sum_of_left_leaves(root), output);
    }

    #[test]
    fn test_sum_of_left_leaves_2() {
        let root = TreeNode::build(vec![Some(1)]);
        let output = 0;
        assert_eq!(Solution::sum_of_left_leaves(root), output);
    }

    #[test]
    fn test_sum_of_left_leaves_3() {
        let root = TreeNode::build(vec![Some(1), Some(2), Some(3), Some(4), Some(5)]);
        println!("{:?}", root);
        let output = 4;
        assert_eq!(Solution::sum_of_left_leaves(root), output);
    }
}
