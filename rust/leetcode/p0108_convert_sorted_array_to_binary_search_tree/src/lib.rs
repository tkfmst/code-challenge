/// 108. Convert Sorted Array to Binary Search Tree
/// Easy
///
/// Given an integer array nums where the elements are sorted in ascending order, convert it to a height-balanced binary search tree.
///
/// A height-balanced binary tree is a binary tree in which the depth of the two subtrees of every node never differs by more than one.
///
///  
///
/// Example 1:
/// ![btree1.jpg](./img/btree1.jpg)
/// Input: nums = [-10,-3,0,5,9]
/// Output: [0,-3,9,-10,null,5]
/// Explanation: [0,-10,5,null,-3,null,9] is also accepted:
/// ![btree2.jpg](./img/btree2.jpg)
///
/// Example 2:
/// ![btree.jpg](./img/btree.jpg)
/// Input: nums = [1,3]
/// Output: [3,1]
/// Explanation: [1,3] and [3,1] are both a height-balanced BSTs.
///
/// Constraints:
///
/// - 1 <= nums.length <= 10^4
/// -10^4 <= nums[i] <= 10^4
/// - nums is sorted in a strictly increasing order.
use std::cell::RefCell;
use std::rc::Rc;
use tools::tree_node::TreeNode;

pub struct Solution {}

impl Solution {
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let len = nums.len();
        if len > 0 {
            let mid = len / 2;
            let l = nums[0..mid].to_vec();
            let v = nums[mid..(mid + 1)].get(0).unwrap(); // len > 0なので必ずSome
            let r = nums[(mid + 1)..len].to_vec();

            Some(Rc::new(RefCell::new(TreeNode {
                val: *v,
                left: Self::sorted_array_to_bst(l),
                right: Self::sorted_array_to_bst(r),
            })))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use tools::tree_node::TreeNode;

    #[test]
    fn test_sorted_array_to_bst1() {
        let input: Vec<i32> = vec![-10, -3, 0, 5, 9];
        let result = TreeNode::build(vec![
            Some(0),
            Some(-3),
            Some(9),
            Some(-10),
            None,
            Some(5),
            None,
        ]);
        assert_eq!(Solution::sorted_array_to_bst(input), result);
    }

    #[test]
    fn test_sorted_array_to_bst2() {
        let input: Vec<i32> = vec![1, 3];
        let result = TreeNode::build(vec![Some(3), Some(1), None]);
        assert_eq!(Solution::sorted_array_to_bst(input), result);
    }
}
