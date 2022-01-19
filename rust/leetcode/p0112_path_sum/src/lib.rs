/// 112. Path Sum
///
/// Given the root of a binary tree and an integer targetSum, return true if the tree has a root-to-leaf path such that adding up all the values along the path equals targetSum.
///
/// A leaf is a node with no children.
///
///
///
/// Example 1:
/// ![pathsum1.jpg](./img/pathsum1.jpg)
///
/// Input: root = [5,4,8,11,null,13,4,7,2,null,null,null,1], targetSum = 22
/// Output: true
/// Explanation: The root-to-leaf path with the target sum is shown.
///
/// Example 2:
/// ![pathsum2.jpg](./img/pathsum2.jpg)
///
/// Input: root = [1,2,3], targetSum = 5
/// Output: false
/// Explanation: There two root-to-leaf paths in the tree:
/// (1 --> 2): The sum is 3.
/// (1 --> 3): The sum is 4.
/// There is no root-to-leaf path with sum = 5.
///
/// Example 3:
///
/// Input: root = [], targetSum = 0
/// Output: false
/// Explanation: Since the tree is empty, there are no root-to-leaf paths.
///
/// Constraints:
///
/// The number of nodes in the tree is in the range [0, 5000].
/// - -1000 <= Node.val <= 1000
/// - -1000 <= targetSum <= 1000
use std::cell::RefCell;
use std::rc::Rc;
use tools::tree_node::TreeNode;

pub struct Solution {}
impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        Self::find(&root, target_sum)
    }

    fn find(root: &Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        if let Some(node) = root {
            match (&node.borrow().left, &node.borrow().right) {
                (Some(_), Some(_)) => {
                    Self::find(&node.borrow().left, target_sum - node.borrow().val)
                        || Self::find(&node.borrow().right, target_sum - node.borrow().val)
                }
                (Some(_), None) => Self::find(&node.borrow().left, target_sum - node.borrow().val),
                (None, Some(_)) => Self::find(&node.borrow().right, target_sum - node.borrow().val),
                (None, None) => target_sum - node.borrow().val == 0,
            }
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use tools::tree_node::TreeNode;
    #[test]
    fn test_has_path_sum1() {
        let input = TreeNode::build(vec![
            Some(5),
            Some(4),
            Some(8),
            Some(11),
            None,
            Some(13),
            Some(4),
            Some(7),
            Some(2),
            None,
            None,
            None,
            Some(1),
        ]);
        let target_sum = 22;
        assert_eq!(Solution::has_path_sum(input, target_sum), true);
    }

    #[test]
    fn test_has_path_sum2() {
        let input = TreeNode::build(vec![Some(1), Some(2), Some(3)]);
        let target_sum = 5;
        assert_eq!(Solution::has_path_sum(input, target_sum), false);
    }

    #[test]
    fn test_has_path_sum3() {
        let input = TreeNode::build(vec![]);
        let target_sum = 0;
        assert_eq!(Solution::has_path_sum(input, target_sum), false);
    }

    #[test]
    fn test_has_path_sum4() {
        let input = TreeNode::build(vec![Some(1), Some(2)]);
        let target_sum = 1;
        assert_eq!(Solution::has_path_sum(input, target_sum), false);
    }

    #[test]
    fn test_has_path_sum5() {
        let input = TreeNode::build(vec![Some(1)]);
        let target_sum = 1;
        assert_eq!(Solution::has_path_sum(input, target_sum), true);
    }

    #[test]
    fn test_has_path_sum6() {
        let input = TreeNode::build(vec![Some(-2), None, Some(-3)]);
        let target_sum = -5;
        assert_eq!(Solution::has_path_sum(input, target_sum), true);
    }

    #[test]
    fn test_has_path_sum7() {
        let input = TreeNode::build(vec![
            Some(1),
            Some(2),
            None,
            Some(3),
            None,
            Some(4),
            None,
            Some(5),
        ]);
        let target_sum = 6;
        assert_eq!(Solution::has_path_sum(input, target_sum), false);
    }
}
