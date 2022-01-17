use std::cell::RefCell;
use std::rc::Rc;
use std::slice::Iter;

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

impl TreeNode {
    pub fn build(input: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut it = input.iter();
        let mut buf: Vec<Option<Rc<RefCell<TreeNode>>>> = vec![];
        if let Some(Some(n)) = it.next() {
            let root = Some(Rc::new(RefCell::new(TreeNode::new(*n))));

            buf.push(root.as_ref().map(|x| Rc::clone(x)));
            Self::build_node(&mut it, buf);

            root
        } else {
            None
        }
    }
    fn build_node(input: &mut Iter<Option<i32>>, nodes: Vec<Option<Rc<RefCell<TreeNode>>>>) {
        let mut buf: Vec<Option<Rc<RefCell<TreeNode>>>> = vec![];

        for op in nodes.iter() {
            if let Some(node) = op {
                let left = match input.next() {
                    Some(Some(n)) => Some(Rc::new(RefCell::new(TreeNode::new(*n)))),
                    _ => None,
                };
                let right = match input.next() {
                    Some(Some(n)) => Some(Rc::new(RefCell::new(TreeNode::new(*n)))),
                    _ => None,
                };
                buf.push(left.as_ref().map(|x| Rc::clone(&x)));
                buf.push(right.as_ref().map(|x| Rc::clone(&x)));

                node.borrow_mut().left = left.map(|x| Rc::clone(&x));
                node.borrow_mut().right = right.map(|x| Rc::clone(&x));
            } else {
                buf.push(None);
                buf.push(None);
            }
        }

        if input.len() > 0 {
            Self::build_node(input, buf);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::TreeNode;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn test_build_0104_1() {
        let input = vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)];
        let result = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 9,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 20,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 15,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 7,
                    left: None,
                    right: None,
                }))),
            }))),
        })));
        assert_eq!(TreeNode::build(input), result)
    }

    #[test]
    fn test_build_0104_2() {
        let input = vec![Some(1), None, Some(2)];
        let result = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: None,
            }))),
        })));
        assert_eq!(TreeNode::build(input), result)
    }

    #[test]
    fn test_build_0110() {
        let input = vec![
            Some(1),
            Some(2),
            Some(2),
            Some(3),
            None,
            None,
            Some(3),
            Some(4),
            None,
            None,
            Some(4),
        ];
        let result = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 4,
                        left: None,
                        right: None,
                    }))),
                    right: None,
                }))),
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 4,
                        left: None,
                        right: None,
                    }))),
                }))),
            }))),
        })));
        assert_eq!(TreeNode::build(input), result)
    }
}
