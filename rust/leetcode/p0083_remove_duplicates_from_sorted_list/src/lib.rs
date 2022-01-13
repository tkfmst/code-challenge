#![allow(dead_code)]

/// 83. Remove Duplicates from Sorted List
///
/// Given the head of a sorted linked list, delete all duplicates such that each element appears only once. Return the linked list sorted as well.
///
///
/// Example 1:
/// ![list1.jpg](./img/list1.jpg)
///
/// Input: head = [1,1,2]
/// Output: [1,2]
///
/// Example 2:
/// ![list1.jpg](./img/list2.jpg)
///
/// Input: head = [1,1,2,3,3]
/// Output: [1,2,3]

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub struct Solution {}

impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return None;
        }

        let mut h = head;

        let mut cur_op = h.as_mut();
        while let Some(cur) = cur_op {
            let mut nxt_op = cur.next.take();

            while let Some(nxt) = nxt_op.as_mut() {
                if nxt.val == cur.val {
                    nxt_op = nxt.next.take();
                } else {
                    cur.next = nxt_op;
                    break;
                }
            }
            cur_op = cur.next.as_mut();
        }
        h
    }
}

#[cfg(test)]
mod tests {
    use super::ListNode;
    use super::Solution;

    #[test]
    fn test_delete_duplicates1() {
        let head = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode { val: 2, next: None })),
            })),
        }));

        let expected = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode { val: 2, next: None })),
        }));

        assert_eq!(Solution::delete_duplicates(head), expected);
    }

    #[test]
    fn test_delete_duplicates2() {
        let head = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode { val: 3, next: None })),
                    })),
                })),
            })),
        }));

        let expected = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode { val: 3, next: None })),
            })),
        }));

        assert_eq!(Solution::delete_duplicates(head), expected);
    }
}
