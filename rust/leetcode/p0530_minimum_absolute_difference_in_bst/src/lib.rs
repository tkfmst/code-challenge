/// 530. Minimum Absolute Difference in BST
/// Easy
///
/// Given the root of a Binary Search Tree (BST), return the minimum absolute difference between the values of any two different nodes in the tree.
///
///  
///
/// Example 1:
///
/// Input: root = [4,2,6,1,3]
/// Output: 1
///
/// Example 2:
///
/// Input: root = [1,0,48,null,null,12,49]
/// Output: 1
///
///  
///
/// Constraints:
/// * The number of nodes in the tree is in the range [2, 10^4].
/// * 0 <= Node.val <= 10^5
use std::cell::RefCell;
use std::rc::Rc;
use tools::tree_node::TreeNode;

pub struct Solution {}
impl Solution {
    pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn in_order_traverse(node: &Option<Rc<RefCell<TreeNode>>>, acc: &mut Vec<i32>) {
            if let Some(node) = node {
                let node = node.borrow();
                in_order_traverse(&node.left, acc);
                acc.push(node.val);
                in_order_traverse(&node.right, acc);
            }
        }

        let mut min = i32::MAX;
        let mut acc = Vec::new();
        in_order_traverse(&root, &mut acc);

        acc.iter()
            .zip(acc[1..].iter())
            .for_each(|(&x, &y)| min = min.min(y - x));

        min
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use tools::tree_node::TreeNode;

    #[test]
    fn test_get_minimum_difference_1() {
        let root = TreeNode::build(vec![Some(4), Some(2), Some(6), Some(1), Some(3)]);
        let output = 1;
        assert_eq!(Solution::get_minimum_difference(root), output);
    }

    #[test]
    fn test_get_minimum_difference_2() {
        let root = TreeNode::build(vec![
            Some(1),
            Some(0),
            Some(48),
            None,
            None,
            Some(12),
            Some(49),
        ]);
        let output = 1;
        assert_eq!(Solution::get_minimum_difference(root), output);
    }
}
