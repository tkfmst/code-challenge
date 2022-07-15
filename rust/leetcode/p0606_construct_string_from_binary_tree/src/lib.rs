/// 606. Construct String from Binary Tree
/// Easy
///
/// Given the root of a binary tree, construct a string consisting of parenthesis and integers from a binary tree with the preorder traversal way, and return it.
///
/// Omit all the empty parenthesis pairs that do not affect the one-to-one mapping relationship between the string and the original binary tree.
///
/// Example 1:
/// ![cons1-tree.jpg](./img/cons1-tree.jpg)
///
/// Input: root = [1,2,3,4]
/// Output: "1(2(4))(3)"
/// Explanation: Originally, it needs to be "1(2(4)())(3()())", but you need to omit all the unnecessary empty parenthesis pairs. And it will be "1(2(4))(3)"
///
/// Example 2:
/// ![cons2-tree.jpg](./img/cons2-tree.jpg)
///
/// Input: root = [1,2,3,null,4]
/// Output: "1(2()(4))(3)"
/// Explanation: Almost the same as the first example, except we cannot omit the first parenthesis pair to break the one-to-one mapping relationship between the input and the output.
///
///  
///
/// Constraints:
///
/// * The number of nodes in the tree is in the range [1, 10^4].
/// * -1000 <= Node.val <= 1000
use std::cell::RefCell;
use std::rc::Rc;
use tools::tree_node::TreeNode;

pub struct Solution {}
impl Solution {
    pub fn tree2str(root: Option<Rc<RefCell<TreeNode>>>) -> String {
        Self::helper(&root)
    }

    fn helper(root: &Option<Rc<RefCell<TreeNode>>>) -> String {
        if let Some(node) = root {
            match (&node.borrow().left, &node.borrow().right) {
                (Some(_), Some(_)) => format!(
                    "{}({})({})",
                    node.borrow().val,
                    Self::helper(&node.borrow().left),
                    Self::helper(&node.borrow().right)
                ),
                (Some(_), None) => format!(
                    "{}({})",
                    node.borrow().val,
                    Self::helper(&node.borrow().left)
                ),
                (None, Some(_)) => format!(
                    "{}()({})",
                    node.borrow().val,
                    Self::helper(&node.borrow().right)
                ),
                (None, None) => format!("{}", node.borrow().val),
            }
        } else {
            "".to_string()
        }
    }
}
#[cfg(test)]
mod tests {
    use super::Solution;
    use tools::tree_node::TreeNode;

    #[test]
    fn test_tree2str_1() {
        let root = TreeNode::build(vec![Some(1), Some(2), Some(3), Some(4)]);
        let output = "1(2(4))(3)".to_string();
        assert_eq!(Solution::tree2str(root), output);
    }

    #[test]
    fn test_tree2str_2() {
        let root = TreeNode::build(vec![Some(1), Some(2), Some(3), None, Some(4)]);
        let output = "1(2()(4))(3)".to_string();
        assert_eq!(Solution::tree2str(root), output);
    }
}
