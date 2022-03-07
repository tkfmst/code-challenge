// 21. Merge Two Sorted Lists
// Easy
//
// You are given the heads of two sorted linked lists list1 and list2.
//
// Merge the two lists in a one sorted list. The list should be made by splicing together the nodes of the first two lists.
//
// Return the head of the merged linked list.
//
//
// Example 1:
// ![merge_ex1.jpg](./img/merge_ex1.jpg)
//
// Input: list1 = [1,2,4], list2 = [1,3,4]
// Output: [1,1,2,3,4,4]
//
// Example 2:
//
// Input: list1 = [], list2 = []
// Output: []
//
// Example 3:
//
// Input: list1 = [], list2 = [0]
// Output: [0]
//
//
// Constraints:
//
// * The number of nodes in both lists is in the range [0, 50].
// * -100 <= Node.val <= 100
// * Both list1 and list2 are sorted in non-decreasing order.

use tools::list_node::ListNode;

pub struct Solution {}

impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (list1, list2) {
            (None, None) => None,
            (l, None) => l,
            (None, r) => r,
            (Some(l), Some(r)) => {
                if l.val < r.val {
                    Some(Box::new(ListNode {
                        val: l.val,
                        next: Solution::merge_two_lists(l.next, Some(r)),
                    }))
                } else {
                    Some(Box::new(ListNode {
                        val: r.clone().val,
                        next: Solution::merge_two_lists(Some(l), r.next),
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
    fn test_merge_two_lists_1() {
        let list1 = ListNode::build(vec![1, 2, 4]);
        let list2 = ListNode::build(vec![1, 3, 4]);
        let output = ListNode::build(vec![1, 1, 2, 3, 4, 4]);
        assert_eq!(Solution::merge_two_lists(list1, list2), output);
    }

    #[test]
    fn test_merge_two_lists_2() {
        let list1 = ListNode::build(vec![]);
        let list2 = ListNode::build(vec![]);
        let output = ListNode::build(vec![]);
        assert_eq!(Solution::merge_two_lists(list1, list2), output);
    }

    #[test]
    fn test_merge_two_lists_3() {
        let list1 = ListNode::build(vec![]);
        let list2 = ListNode::build(vec![0]);
        let output = ListNode::build(vec![0]);
        assert_eq!(Solution::merge_two_lists(list1, list2), output);
    }
}
