/// 637. Average of Levels in Binary Tree
/// Easy
/// Given the root of a binary tree, return the average value of the nodes on each level in the form of an array. Answers within 10-5 of the actual answer will be accepted.
///
///
/// Example 1:
/// ![avg1-tree.jpg](./img/avg1-tree.jpg)
///
/// Input: root = [3,9,20,null,null,15,7]
/// Output: [3.00000,14.50000,11.00000]
/// Explanation: The average value of nodes on level 0 is 3, on level 1 is 14.5, and on level 2 is 11.
/// Hence return [3, 14.5, 11].
///
/// Example 2:
/// ![avg2-tree.jpg](./img/avg2-tree.jpg)
///
/// Input: root = [3,9,20,15,7]
/// Output: [3.00000,14.50000,11.00000]
///
///
/// Constraints:
/// * The number of nodes in the tree is in the range [1, 10^4].
/// * -2^31 <= Node.val <= 2^31 - 1
use std::cell::RefCell;
use std::rc::Rc;
use tools::tree_node::TreeNode;

pub struct Solution {}

impl Solution {
    pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
        let mut map: Vec<Vec<i32>> = Vec::with_capacity(10000);

        Solution::helper(&root, 0, &mut map);

        let mut result: Vec<f64> = Vec::with_capacity(10000);
        for v in map {
            result.push(v.iter().map(|&x| x as f64).sum::<f64>() / (v.len() as f64));
        }

        result
    }

    fn helper(root: &Option<Rc<RefCell<TreeNode>>>, depth: usize, map: &mut Vec<Vec<i32>>) -> () {
        if let Some(node) = root {
            while map.len() < depth + 1 {
                map.push(vec![]);
            }
            match (&node.borrow().left, &node.borrow().right) {
                (Some(_), Some(_)) => {
                    map[depth].push(node.borrow().val.clone());
                    Self::helper(&node.borrow().left, depth + 1, map);
                    Self::helper(&node.borrow().right, depth + 1, map);
                }
                (Some(_), None) => {
                    map[depth].push(node.borrow().val.clone());
                    Self::helper(&node.borrow().left, depth + 1, map);
                }
                (None, Some(_)) => {
                    map[depth].push(node.borrow().val.clone());
                    Self::helper(&node.borrow().right, depth + 1, map);
                }
                (None, None) => {
                    map[depth].push(node.borrow().val.clone());
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
    fn test_average_of_levels_1() {
        let root = TreeNode::build(vec![
            Some(3),
            Some(9),
            Some(20),
            None,
            None,
            Some(15),
            Some(7),
        ]);
        let output = vec![3f64, 14.5, 11.0];
        assert_eq!(Solution::average_of_levels(root), output);
    }

    #[test]
    fn test_average_of_levels_2() {
        let root = TreeNode::build(vec![Some(3), Some(9), Some(20), Some(15), Some(7)]);
        let output = vec![3f64, 14.5, 11.0];
        assert_eq!(Solution::average_of_levels(root), output);
    }
}
