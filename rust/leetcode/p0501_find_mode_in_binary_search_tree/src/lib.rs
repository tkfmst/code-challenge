/// 501. Find Mode in Binary Search Tree
/// Easy
///
/// Given the root of a binary search tree (BST) with duplicates, return all the mode(s) (i.e., the most frequently occurred element) in it.
///
/// If the tree has more than one mode, return them in any order.
///
/// Assume a BST is defined as follows:
///
/// The left subtree of a node contains only nodes with keys less than or equal to the node's key.
/// The right subtree of a node contains only nodes with keys greater than or equal to the node's key.
/// Both the left and right subtrees must also be binary search trees.
///
///
/// Example 1:
/// ![node-tree](./img/mode-tree.jpg)
///
/// Input: root = [1,null,2,2]
/// Output: [2]
///
/// Example 2:
///
/// Input: root = [0]
/// Output: [0]
///
///
/// Constraints:
/// * The number of nodes in the tree is in the range [1, 10^4].
/// * -10^5 <= Node.val <= 10^5
///
/// Follow up: Could you do that without using any extra space? (Assume that the implicit stack space incurred due to recursion does not count).
///
use std::cell::RefCell;
use std::rc::Rc;
use tools::tree_node::TreeNode;

use std::collections::HashMap;

pub struct Solution {}
impl Solution {
    pub fn find_mode(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut max_mode = 0;
        let mut res = vec![];

        let mut stack = vec![root];
        let mut counter = HashMap::new();

        while let Some(op) = stack.pop() {
            if let Some(node) = op {
                let num = counter.entry(node.borrow().val).or_insert(0);
                *num += 1;

                if *num >= max_mode {
                    if *num > max_mode {
                        res.clear();
                        max_mode = *num;
                    }
                    res.push(node.borrow().val);
                }

                stack.push(node.borrow().left.clone());
                stack.push(node.borrow().right.clone());
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use tools::tree_node::TreeNode;

    #[test]
    fn test_find_mode_1() {
        let root = TreeNode::build(vec![Some(1), None, Some(2), Some(2)]);
        let output = vec![2];
        assert_eq!(Solution::find_mode(root), output);
    }

    #[test]
    fn test_find_mode_2() {
        let root = TreeNode::build(vec![Some(0)]);
        let output = vec![0];
        assert_eq!(Solution::find_mode(root), output);
    }
}
