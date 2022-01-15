use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    pub fn build(input: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        let idx: u32 = 0;
        if let Some(Some(n)) = input.get(0) {
            Self::build_node(idx, *n, &input)
        } else {
            None
        }
    }

    fn build_node(idx: u32, val: i32, input: &Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        Some(Rc::new(RefCell::new(TreeNode {
            val: val,
            left: Self::build_left(idx, input),
            right: Self::build_right(idx, input),
        })))
    }

    fn build_left(parent_idx: u32, input: &Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        let idx = parent_idx * 2 + 1;
        if let Some(Some(n)) = input.get(idx as usize) {
            Self::build_node(idx, *n, input)
        } else {
            None
        }
    }

    fn build_right(parent_idx: u32, input: &Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        let idx = parent_idx * 2 + 2;
        if let Some(Some(n)) = input.get(idx as usize) {
            Self::build_node(idx, *n, input)
        } else {
            None
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
}
