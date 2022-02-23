/// 235. Lowest Common Ancestor of a Binary Search Tree
/// Easy
///
/// Given a binary search tree (BST), find the lowest common ancestor (LCA) of two given nodes in the BST.
///
/// According to the definition of LCA on Wikipedia: “The lowest common ancestor is defined between two nodes p and q as the lowest node in T that has both p and q as descendants (where we allow a node to be a descendant of itself).”
///
/// Example 1:
/// ![binarysearchtree_improved.png](./img/binarysearchtree_improved.png)
///
/// Input: root = [6,2,8,0,4,7,9,null,null,3,5], p = 2, q = 8
/// Output: 6
/// Explanation: The LCA of nodes 2 and 8 is 6.
///
/// Example 2:
/// ![binarysearchtree_improved.png](./img/binarysearchtree_improved.png)
///
/// Input: root = [6,2,8,0,4,7,9,null,null,3,5], p = 2, q = 4
/// Output: 2
/// Explanation: The LCA of nodes 2 and 4 is 2, since a node can be a descendant of itself according to the LCA definition.
///
/// Example 3:
///
/// Input: root = [2,1], p = 2, q = 1
/// Output: 2
///
///
///
/// Constraints:
///
/// * The number of nodes in the tree is in the range [2, 10^5].
/// * -10^9 <= Node.val <= 10^9
/// * All Node.val are unique.
/// * p != q
/// * p and q will exist in the BST.
use std::cell::RefCell;
use std::rc::Rc;
use tools::tree_node::TreeNode;

pub struct Solution {}

impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match (root, p, q) {
            (Some(root2), Some(p2), Some(q2)) => {
                if root2.borrow().val == p2.borrow().val || root2.borrow().val == q2.borrow().val {
                    Some(root2.clone())
                } else {
                    match (
                        p2.borrow().val < root2.borrow().val,
                        q2.borrow().val < root2.borrow().val,
                    ) {
                        (true, true) => Solution::lowest_common_ancestor(
                            root2.borrow().left.clone(),
                            Some(p2.clone()),
                            Some(q2.clone()),
                        ),
                        (false, false) => Solution::lowest_common_ancestor(
                            root2.borrow().right.clone(),
                            Some(p2.clone()),
                            Some(q2.clone()),
                        ),
                        _ => Some(root2.clone()),
                    }
                }
            }
            _ => {
                panic!("Unreachable")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use std::cell::RefCell;
    use std::rc::Rc;
    use tools::tree_node::TreeNode;

    #[test]
    fn test_lowest_common_ancestor_1() {
        let root = TreeNode::build(vec![
            Some(6),
            Some(2),
            Some(8),
            Some(0),
            Some(4),
            Some(7),
            Some(9),
            None,
            None,
            Some(3),
            Some(5),
        ]);
        let p = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        let q = Some(Rc::new(RefCell::new(TreeNode::new(8))));
        let output = TreeNode::build(vec![
            Some(6),
            Some(2),
            Some(8),
            Some(0),
            Some(4),
            Some(7),
            Some(9),
            None,
            None,
            Some(3),
            Some(5),
        ]);
        assert_eq!(Solution::lowest_common_ancestor(root, p, q), output);
    }

    #[test]
    fn test_lowest_common_ancestor_2() {
        let root = TreeNode::build(vec![
            Some(6),
            Some(2),
            Some(8),
            Some(0),
            Some(4),
            Some(7),
            Some(9),
            None,
            None,
            Some(3),
            Some(5),
        ]);
        let p = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        let q = Some(Rc::new(RefCell::new(TreeNode::new(4))));
        let output = TreeNode::build(vec![
            Some(2),
            Some(0),
            Some(4),
            None,
            None,
            Some(3),
            Some(5),
        ]);
        assert_eq!(Solution::lowest_common_ancestor(root, p, q), output);
    }

    #[test]
    fn test_lowest_common_ancestor_3() {
        let root = TreeNode::build(vec![Some(2), Some(1)]);
        let p = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        let q = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        let output = TreeNode::build(vec![Some(2), Some(1)]);
        assert_eq!(Solution::lowest_common_ancestor(root, p, q), output);
    }
}
