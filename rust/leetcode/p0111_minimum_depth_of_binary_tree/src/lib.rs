/// 111. Minimum Depth of Binary Tree
///
/// Given a binary tree, find its minimum depth.
///
/// The minimum depth is the number of nodes along the shortest path from the root node down to the nearest leaf node.
///
/// Note: A leaf is a node with no children.
///
/// Example 1:
/// ![ex_depth.jpg](./img/ex_depth.jpg)
///
/// Input: root = [3,9,20,null,null,15,7]
/// Output: 2
///
/// Example 2:
///
/// Input: root = [2,null,3,null,4,null,5,null,6]
/// Output: 5
///
///  
///
/// Constraints:
/// - The number of nodes in the tree is in the range [0, 105].
/// - -1000 <= Node.val <= 1000
///
use std::cell::RefCell;
use std::rc::Rc;
use tools::tree_node::TreeNode;

pub struct Solution {}
impl Solution {
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::find_min_depth(&root)
    }
    fn find_min_depth(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            None => 0,
            Some(node) => {
                let left = Solution::find_min_depth(&node.borrow().left);
                let right = Solution::find_min_depth(&node.borrow().right);

                match (left, right) {
                    (0, 0) => 1,
                    (0, _) => right + 1,
                    (_, 0) => left + 1,
                    _ => {
                        if left < right {
                            left + 1
                        } else {
                            right + 1
                        }
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
    fn test_min_depth1() {
        let input = TreeNode::build(vec![
            Some(3),
            Some(9),
            Some(20),
            None,
            None,
            Some(15),
            Some(7),
        ]);
        assert_eq!(Solution::min_depth(input), 2);
    }

    #[test]
    fn test_min_depth2() {
        let input = TreeNode::build(vec![
            Some(2),
            None,
            Some(3),
            None,
            Some(4),
            None,
            Some(5),
            None,
            Some(6),
        ]);
        assert_eq!(Solution::min_depth(input), 5);
    }
}
