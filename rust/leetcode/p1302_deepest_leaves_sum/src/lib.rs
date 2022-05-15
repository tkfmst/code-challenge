/// 1302. Deepest Leaves Sum
/// Medium
/// Given the root of a binary tree, return the sum of values of its deepest leaves.
///
///
/// Example 1:
/// ![1483_ex1.png](./img/1483_ex1.png)
///
/// Input: root = [1,2,3,4,5,null,6,7,null,null,null,null,8]
/// Output: 15
///
/// Example 2:
///
/// Input: root = [6,7,8,2,7,1,3,9,null,1,4,null,null,null,5]
/// Output: 19
///
///
/// Constraints:
///
/// * the number of nodes in the tree is in the range [1, 104].
/// * 1 <= Node.val <= 100
use std::cell::RefCell;
use std::rc::Rc;
use tools::tree_node::TreeNode;

pub struct Solution {}
impl Solution {
    pub fn deepest_leaves_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max_depth = 0;
        let mut deepest_vals = 0;

        Self::find(&root, 0, &mut max_depth, &mut deepest_vals);

        deepest_vals
    }
    fn find(
        root: &Option<Rc<RefCell<TreeNode>>>,
        prev_depth: i32,
        max_depth: &mut i32,
        deepest_vals: &mut i32,
    ) {
        if let Some(node) = root {
            let depth = prev_depth + 1;
            if depth > *max_depth {
                *max_depth += 1;
                *deepest_vals = 0;
            }
            match (&node.borrow().left, &node.borrow().right) {
                (Some(_), Some(_)) => {
                    Self::find(&node.borrow().left, depth, max_depth, deepest_vals);
                    Self::find(&node.borrow().right, depth, max_depth, deepest_vals);
                }
                (Some(_), None) => {
                    Self::find(&node.borrow().left, depth, max_depth, deepest_vals);
                }
                (None, Some(_)) => {
                    Self::find(&node.borrow().right, depth, max_depth, deepest_vals);
                }
                (None, None) => {
                    if depth == *max_depth {
                        *deepest_vals += node.borrow().val;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use tools::tree_node::TreeNode;

    #[test]
    fn test_deepest_leaves_sum_1() {
        let root = TreeNode::build(vec![
            Some(1),
            Some(2),
            Some(3),
            Some(4),
            Some(5),
            None,
            Some(6),
            Some(7),
            None,
            None,
            None,
            None,
            Some(8),
        ]);
        let output = 15;
        assert_eq!(Solution::deepest_leaves_sum(root), output);
    }

    #[test]
    fn test_deepest_leaves_sum_2() {
        let root = TreeNode::build(vec![
            Some(6),
            Some(7),
            Some(8),
            Some(2),
            Some(7),
            Some(1),
            Some(3),
            Some(9),
            None,
            Some(1),
            Some(4),
            None,
            None,
            None,
            Some(5),
        ]);
        let output = 19;
        assert_eq!(Solution::deepest_leaves_sum(root), output);
    }
}
