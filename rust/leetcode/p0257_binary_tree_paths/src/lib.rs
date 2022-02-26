/// 257. Binary Tree Paths
/// Easy
///
/// Given the root of a binary tree, return all root-to-leaf paths in any order.
///
/// A leaf is a node with no children.
///
/// Example 1:
/// ![paths-tree.jpg](./img/paths-tree.jpg)
///
/// Input: root = [1,2,3,null,5]
/// Output: ["1->2->5","1->3"]
///
/// Example 2:
///
/// Input: root = [1]
/// Output: ["1"]
///
/// Constraints:
///
/// * The number of nodes in the tree is in the range [1, 100].
/// * -100 <= Node.val <= 100
use std::cell::RefCell;
use std::rc::Rc;
use tools::tree_node::TreeNode;

pub struct Solution {}

impl Solution {
    pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
        let mut result: Vec<String> = vec![];
        for v in Self::path(&root) {
            let s: String = match v.as_slice().split_first() {
                Some((head, tail)) => tail
                    .iter()
                    .fold(head.to_string(), |acc, i| format!("{}->{}", acc, i)),
                None => "".to_string(),
            };
            result.push(s);
        }
        result
    }
    fn path(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut v: Vec<Vec<i32>> = vec![];
        if let Some(node) = root {
            match (&node.borrow().left, &node.borrow().right) {
                (Some(_), Some(_)) => {
                    let mut v1 = Self::helper(&node.borrow().left, node.borrow().val);
                    let mut v2 = Self::helper(&node.borrow().right, node.borrow().val);
                    v1.append(&mut v2);
                    v = v1;
                }
                (Some(_), None) => {
                    v = Self::helper(&node.borrow().left, node.borrow().val);
                }
                (None, Some(_)) => {
                    v = Self::helper(&node.borrow().right, node.borrow().val);
                }
                (None, None) => v.push(vec![node.borrow().val]),
            }
        }
        v
    }
    fn helper(next: &Option<Rc<RefCell<TreeNode>>>, val: i32) -> Vec<Vec<i32>> {
        let mut v: Vec<Vec<i32>> = vec![];
        for path in Self::path(next).iter_mut() {
            path.insert(0, val);
            v.push(path.to_vec());
        }
        v
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use tools::tree_node::TreeNode;

    #[test]
    fn test_binary_tree_paths_1() {
        let root = TreeNode::build(vec![Some(1), Some(2), Some(3), None, Some(5)]);
        let output = vec!["1->2->5", "1->3"];
        assert_eq!(Solution::binary_tree_paths(root), output);
    }

    #[test]
    fn test_binary_tree_paths_2() {
        let root = TreeNode::build(vec![Some(1)]);
        let output = vec!["1"];
        assert_eq!(Solution::binary_tree_paths(root), output);
    }
}
