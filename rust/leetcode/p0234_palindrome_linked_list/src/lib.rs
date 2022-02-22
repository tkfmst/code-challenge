/// 234. Palindrome Linked List
/// Easy
///
/// Given the head of a singly linked list, return true if it is a palindrome.
///
/// Example 1:
/// ![pal1linked-list.jpg](./img/pal1linked-list.jpg)
///
/// Input: head = [1,2,2,1]
/// Output: true
///
/// Example 2:
/// ![pal2linked-list.jpg](./img/pal1linked-list.jpg)
///
/// Input: head = [1,2]
/// Output: false
///
///  
///
/// Constraints:
/// * The number of nodes in the list is in the range [1, 10^5].
/// * 0 <= Node.val <= 9
///
///  
/// Follow up: Could you do it in O(n) time and O(1) space?
use tools::list_node::ListNode;

pub struct Solution {}

impl Solution {
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        let mut cur = head;
        // ListNodeでVec使うのはありなのか？
        let mut v: Vec<i32> = vec![];

        while let Some(node) = cur {
            v.push(node.val);
            cur = node.next;
        }

        let l = v.len();

        (l == 1)
            || (l > 1)
                && (v[..l / 2]
                    .iter()
                    .zip(v[l / 2..].iter().rev())
                    .all(|(x, y)| x == y))
    }

    //
    // // Time Limit Exceeded :(
    //
    // pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
    //     let middle = Self::mid(&head);
    //     let (mut left, mut right) = (&head, &Self::rev(middle));
    //
    //     while left != middle {
    //         if let (Some(l), Some(r)) = (left, right) {
    //             if l.val != r.val {
    //                 return false;
    //             }
    //
    //             left = &l.next;
    //             right = &r.next;
    //         } else {
    //             return false;
    //         }
    //     }
    //
    //     true
    // }
    //
    // fn mid(head: &Option<Box<ListNode>>) -> &Option<Box<ListNode>> {
    //     let mut slow = head;
    //     let mut fast = head;
    //
    //     while let (Some(s), Some(f)) = (slow, fast) {
    //         if let Some(fnext) = &f.next {
    //             slow = &s.next;
    //             fast = &fnext.next;
    //         }
    //     }
    //
    //     slow
    // }
    //
    // fn rev(head: &Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    //     let mut cur = head;
    //     let mut prev = None;
    //     while let Some(node) = cur {
    //         cur = &node.next;
    //         prev = Some(Box::new(ListNode {
    //             val: node.val,
    //             next: prev,
    //         }));
    //     }
    //
    //     prev
    // }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use tools::list_node::ListNode;

    #[test]
    fn test_is_palindrome_1() {
        let input = ListNode::build(vec![1, 2, 2, 1]);
        let output = true;
        assert_eq!(Solution::is_palindrome(input), output);
    }

    #[test]
    fn test_is_palindrome_2() {
        let input = ListNode::build(vec![1, 2]);
        let output = false;
        assert_eq!(Solution::is_palindrome(input), output);
    }
}
