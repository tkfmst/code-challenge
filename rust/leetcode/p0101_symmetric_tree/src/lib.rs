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
/// * -100 <= Node.val <= 100
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

impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match root {
            Some(node) => {
                let left = Rc::clone(&node)
                    .borrow()
                    .left
                    .as_ref()
                    .map(|x| Rc::clone(&x));
                let right = Rc::clone(&node)
                    .borrow()
                    .right
                    .as_ref()
                    .map(|x| Rc::clone(&x));

                left == right && is_symmetric_equal(left, right)
            }
            None => false,
        }
    }
}

fn is_symmetric_equal(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
    match (p, q) {
        (Some(_p), Some(_q)) => {
            let p_left = Rc::clone(&_p).borrow().left.as_ref().map(|x| Rc::clone(&x));
            let p_right = Rc::clone(&_p)
                .borrow()
                .right
                .as_ref()
                .map(|x| Rc::clone(&x));
            let q_left = Rc::clone(&_q).borrow().left.as_ref().map(|x| Rc::clone(&x));
            let q_right = Rc::clone(&_q)
                .borrow()
                .right
                .as_ref()
                .map(|x| Rc::clone(&x));

            match (p_left, q_right) {
                (Some(p_l), Some(q_r) => Rc::clone(&p)&_l.val == q_r.val,
                (None, None) => true,
                _ => false,
            };
            p_left == q_right
                && p_right == q_left
                && is_symmetric_equal(p_left, q_right)
                && is_symmetric_equal(p_right, q_left)
        }
        (None, None) => true,
        _ => false,
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
