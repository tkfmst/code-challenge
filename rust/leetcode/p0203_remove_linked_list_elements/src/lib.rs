/// 203. Remove Linked List Elements
/// Easy
///
/// Given the head of a linked list and an integer val, remove all the nodes of the linked list that has Node.val == val, and return the new head.
///
///
/// Example 1:
/// ![removelinked-list.jpg](./img/removelinked-list.jpg)
/// Input: head = [1,2,6,3,4,5,6], val = 6
/// Output: [1,2,3,4,5]
///
/// Example 2:
///
/// Input: head = [], val = 1
/// Output: []
///
/// Example 3:
///
/// Input: head = [7,7,7,7], val = 7
/// Output: []
///
///
/// Constraints:
/// * The number of nodes in the list is in the range [0, 10^4].
/// * 1 <= Node.val <= 50
/// * 0 <= val <= 50
use tools::list_node::ListNode;

pub struct Solution {}

impl Solution {
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        match head {
            None => None,
            Some(node) => {
                if node.val == val {
                    Self::remove_elements(node.next, val)
                } else {
                    Some(Box::new(ListNode {
                        val: node.val,
                        next: Self::remove_elements(node.next, val),
                    }))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use tools::list_node::ListNode;

    #[test]
    fn test_invert_tree_1() {
        let input = ListNode::build(vec![1, 2, 6, 3, 4, 5, 6]);
        let target = 6;
        let output = ListNode::build(vec![1, 2, 3, 4, 5]);
        assert_eq!(Solution::remove_elements(input, target), output);
    }

    #[test]
    fn test_invert_tree_2() {
        let input = ListNode::build(vec![]);
        let target = 1;
        let output = ListNode::build(vec![]);
        assert_eq!(Solution::remove_elements(input, target), output);
    }

    #[test]
    fn test_invert_tree_3() {
        let input = ListNode::build(vec![7, 7, 7, 7]);
        let target = 7;
        let output = ListNode::build(vec![]);
        assert_eq!(Solution::remove_elements(input, target), output);
    }
}
