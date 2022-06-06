/// 543. Diameter of Binary Tree
/// Easy
///
/// Given the root of a binary tree, return the length of the diameter of the tree.
///
/// The diameter of a binary tree is the length of the longest path between any two nodes in a tree. This path may or may not pass through the root.
///
/// The length of a path between two nodes is represented by the number of edges between them.
///
///  
///
/// Example 1:
///
/// Input: root = [1,2,3,4,5]
/// Output: 3
/// Explanation: 3 is the length of the path [4,2,1,3] or [5,2,1,3].
///
/// Example 2:
///
/// Input: root = [1,2]
/// Output: 1
///
///  
///
/// Constraints:
/// * The number of nodes in the tree is in the range [1, 10^4].
/// * -100 <= Node.val <= 100
use std::cell::RefCell;
use std::rc::Rc;
use tools::tree_node::TreeNode;

pub struct Solution {}
impl Solution {
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn depth(root: &Option<Rc<RefCell<TreeNode>>>, res: &mut i32) -> i32 {
            root.as_ref()
                .map(|n| n.borrow())
                .map(|n| {
                    let (l, r) = (depth(&n.left, res), depth(&n.right, res));
                    *res = (*res).max(l + r + 2);
                    return 1 + l.max(r);
                })
                .unwrap_or(-1)
        }
        let mut res = 0;
        depth(&root, &mut res);
        res
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use tools::tree_node::TreeNode;

    #[test]
    fn test_diameter_of_binary_tree_1() {
        let root = TreeNode::build(vec![Some(1), Some(2), Some(3), Some(4), Some(5)]);
        let output = 3;
        assert_eq!(Solution::diameter_of_binary_tree(root), output);
    }

    #[test]
    fn test_diameter_of_binary_tree_2() {
        let root = TreeNode::build(vec![Some(1), Some(2)]);
        let output = 1;
        assert_eq!(Solution::diameter_of_binary_tree(root), output);
    }
}
