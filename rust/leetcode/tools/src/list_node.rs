#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    pub fn build(input: Vec<i32>) -> Option<Box<Self>> {
        match input.split_first() {
            None => None,
            Some((head, tail)) => Some(Box::new(ListNode {
                val: *head,
                next: Self::build(tail.to_vec()),
            })),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ListNode;

    #[test]
    fn test_build_0203_1() {
        let input = vec![1, 2, 6, 3, 4, 5, 6];
        let result = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 6,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode {
                            val: 4,
                            next: Some(Box::new(ListNode {
                                val: 5,
                                next: Some(Box::new(ListNode { val: 6, next: None })),
                            })),
                        })),
                    })),
                })),
            })),
        }));
        assert_eq!(ListNode::build(input), result)
    }

    #[test]
    fn test_build_0203_2() {
        let input = vec![];
        let result = None;
        assert_eq!(ListNode::build(input), result)
    }
}
