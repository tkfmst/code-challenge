/// 100.SameTree
///
/// Given the roots of two binary trees p and q, write a function to check if they are the same or not.
///
/// Two binary trees are considered the same if they are structurally identical, and the nodes have the same value.
///
///
///
/// Example 1:
/// ![ex1](./img/ex1.jpg)
/// Input: p = [1,2,3], q = [1,2,3]
/// Output: true
///
/// Example 2:
/// ![ex1](./img/ex2.jpg)
/// Input: p = [1,2], q = [1,null,2]
/// Output: false
///
/// Example 3:
/// ![ex1](./img/ex3.jpg)
/// Input: p = [1,2,1], q = [1,1,2]
/// Output: false
use std::cell::RefCell;
use std::rc::Rc;

// Definition for a binary tree node.
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

impl Solution {
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (p, q) {
            (Some(_p), Some(_q)) => {
                Rc::clone(&_p).borrow().val == Rc::clone(&_q).borrow().val
                    && Self::is_same_tree(
                        Rc::clone(&_p).borrow().left.as_ref().map(|x| Rc::clone(&x)),
                        Rc::clone(&_q).borrow().left.as_ref().map(|x| Rc::clone(&x)),
                    )
                    && Self::is_same_tree(
                        Rc::clone(&_p)
                            .borrow()
                            .right
                            .as_ref()
                            .map(|x| Rc::clone(&x)),
                        Rc::clone(&_q)
                            .borrow()
                            .right
                            .as_ref()
                            .map(|x| Rc::clone(&x)),
                    )
            }
            (None, None) => true,
            _ => return false,
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
    fn test_is_same_tree1() {
        let p = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            }))),
        })));
        let q = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            }))),
        })));
        assert_eq!(Solution::is_same_tree(p, q), true);
    }

    #[test]
    fn test_is_same_tree2() {
        let p = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: None,
            }))),
            right: None,
        })));
        let q = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: None,
            }))),
        })));
        assert_eq!(Solution::is_same_tree(p, q), false);
    }

    #[test]
    fn test_is_same_tree3() {
        let p = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: None,
            }))),
        })));
        let q = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: None,
            }))),
        })));
        assert_eq!(Solution::is_same_tree(p, q), false);
    }
}
