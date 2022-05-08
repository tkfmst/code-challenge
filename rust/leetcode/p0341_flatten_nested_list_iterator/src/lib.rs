/// 341. Flatten Nested List Iterator
/// Medium
///
/// You are given a nested list of integers nestedList. Each element is either an integer or a list whose elements may also be integers or other lists. Implement an iterator to flatten it.
///
/// Implement the NestedIterator class:
///
/// * NestedIterator(List<NestedInteger> nestedList) Initializes the iterator with the nested list nestedList.
/// * int next() Returns the next integer in the nested list.
/// * boolean hasNext() Returns true if there are still some integers in the nested list and false otherwise.
///
/// Your code will be tested with the following pseudocode:
///
/// ```pseudocode
/// initialize iterator with nestedList
/// res = []
/// while iterator.hasNext()
/// append iterator.next() to the end of res
/// return res
/// ```
///
/// If res matches the expected flattened list, then your code will be judged as correct.
///
///
/// Example 1:
///
/// Input: nestedList = [[1,1],2,[1,1]]
/// Output: [1,1,2,1,1]
/// Explanation: By calling next repeatedly until hasNext returns false, the order of elements returned by next should be: [1,1,2,1,1].
///
/// Example 2:
///
/// Input: nestedList = [1,[4,[6]]]
/// Output: [1,4,6]
/// Explanation: By calling next repeatedly until hasNext returns false, the order of elements returned by next should be: [1,4,6].
///
///
/// Constraints:
/// * 1 <= nestedList.length <= 500
/// * The values of the integers in the nested list is in the range [-106, 106].

#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>),
}

pub struct NestedIterator {
    val: Vec<i32>,
}

impl NestedIterator {
    fn flat_nested_interger(res: &mut Vec<i32>, nested_list: Vec<NestedInteger>) {
        for ni in nested_list {
            match ni {
                NestedInteger::Int(i) => res.push(i),
                NestedInteger::List(v) => Self::flat_nested_interger(res, v),
            }
        }
    }

    #[allow(non_snake_case)]
    pub fn new(nestedList: Vec<NestedInteger>) -> Self {
        let mut res = vec![];
        Self::flat_nested_interger(&mut res, nestedList);

        Self { val: res }
    }

    pub fn next(&mut self) -> i32 {
        self.val.drain(0..1).last().unwrap()
    }

    pub fn has_next(&self) -> bool {
        !self.val.is_empty()
    }
}

/**
 * Your NestedIterator object will be instantiated and called as such:
 * let obj = NestedIterator::new(nestedList);
 * let ret_1: i32 = obj.next();
 * let ret_2: bool = obj.has_next();
 */

#[cfg(test)]
mod tests {
    use super::NestedInteger;
    use super::NestedInteger::Int;
    use super::NestedInteger::List;
    use super::NestedIterator;

    fn run_test(nested_list: Vec<NestedInteger>) -> Vec<i32> {
        let mut it = NestedIterator::new(nested_list);
        let mut res = vec![];
        while it.has_next() {
            res.push(it.next());
        }
        res
    }

    #[test]
    fn test_nested_list_iterator_1() {
        let input = vec![
            List(vec![Int(1), Int(1)]),
            Int(2),
            List(vec![Int(1), Int(1)]),
        ];
        let output = vec![1, 1, 2, 1, 1];
        assert_eq!(run_test(input), output);
    }

    #[test]
    fn test_nested_list_iterator_2() {
        let input = vec![List(vec![Int(1), List(vec![Int(4), List(vec![Int(6)])])])];
        let output = vec![1, 4, 6];
        assert_eq!(run_test(input), output);
    }
}
