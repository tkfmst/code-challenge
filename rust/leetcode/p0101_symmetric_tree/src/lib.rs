/// 101. Symmetric Tree
/// Easy
///
/// Given the root of a binary tree, check whether it is a mirror of itself (i.e., symmetric around its center).
///
///  
///
/// Example 1:
/// ![symtree1](./img/symtree1.jpg)
///
/// Input: root = [1,2,2,3,4,4,3]
/// Output: true
///
/// Example 2:
/// ![symtree1](./img/symtree2.jpg)
///
/// Input: root = [1,2,2,null,3,null,3]
/// Output: false
///
///  
///
/// Constraints:
/// * The number of nodes in the tree is in the range [1, 1000].
/// * -100 <= Link.val <= 100
///
/// Follow up: Could you solve it both recursively and iteratively?
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

pub struct Solution {}

type Link = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match root {
            None => true,
            Some(node) => Self::symmetric_equals(&node.borrow().left, &node.borrow().right),
        }
    }

    fn symmetric_equals(p: &Link, q: &Link) -> bool {
        match (p, q) {
            (None, None) => true,
            (Some(pnode), Some(qnode)) => {
                pnode.borrow().val == qnode.borrow().val
                    && Self::symmetric_equals(&pnode.borrow().left, &qnode.borrow().right)
                    && Self::symmetric_equals(&pnode.borrow().right, &qnode.borrow().left)
            }
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use super::TreeNode;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn test_is_symmetric1() {
        let tree = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None,
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: None,
                }))),
            }))),
        })));
        assert_eq!(Solution::is_symmetric(tree), true);
    }

    #[test]
    fn test_is_symmetric2() {
        let tree = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: None,
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: None,
                }))),
            }))),
        })));
        assert_eq!(Solution::is_symmetric(tree), false);
    }
}
