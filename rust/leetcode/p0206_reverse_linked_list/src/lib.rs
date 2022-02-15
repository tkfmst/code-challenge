/// 206. Reverse Linked List
/// Easy
///
/// Given the head of a singly linked list, reverse the list, and return the reversed list.
///
///  
///
/// Example 1:
/// ![rev1ex1.jpg](./img/rev1ex1.jpg)
///
/// Input: head = [1,2,3,4,5]
/// Output: [5,4,3,2,1]
///
/// Example 2:
/// ![rev1ex2.jpg](./img/rev1ex2.jpg)
///
/// Input: head = [1,2]
/// Output: [2,1]
///
/// Example 3:
///
/// Input: head = []
/// Output: []
///
///  
///
/// Constraints:
/// * The number of nodes in the list is the range [0, 5000].
/// * -5000 <= Node.val <= 5000
///
/// Follow up: A linked list can be reversed either iteratively or recursively. Could you implement both?
use tools::list_node::ListNode;

pub struct Solution {}

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut input = head;
        let mut output: Option<Box<ListNode>> = None;
        while let Some(node) = input {
            output = Some(Box::new(ListNode {
                val: node.val,
                next: output,
            }));
            input = node.next;
        }
        output
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use tools::list_node::ListNode;

    #[test]
    fn test_reverse_list_1() {
        let input = ListNode::build(vec![1, 2, 3, 4, 5]);
        let output = ListNode::build(vec![5, 4, 3, 2, 1]);
        assert_eq!(Solution::reverse_list(input), output);
    }

    #[test]
    fn test_reverse_list_2() {
        let input = ListNode::build(vec![1, 2]);
        let output = ListNode::build(vec![2, 1]);
        assert_eq!(Solution::reverse_list(input), output);
    }

    #[test]
    fn test_reverse_list_3() {
        let input = ListNode::build(vec![]);
        let output = ListNode::build(vec![]);
        assert_eq!(Solution::reverse_list(input), output);
    }
}
