/// 705. Design HashSet
/// Easy
///
/// Design a HashSet without using any built-in hash table libraries.
///
/// Implement MyHashSet class:
///
/// void add(key) Inserts the value key into the HashSet.
/// bool contains(key) Returns whether the value key exists in the HashSet or not.
/// void remove(key) Removes the value key in the HashSet. If key does not exist in the HashSet, do nothing.
///
///
/// Example 1:
///
/// Input
/// ["MyHashSet", "add", "add", "contains", "contains", "add", "contains", "remove", "contains"]
/// [[], [1], [2], [1], [3], [2], [2], [2], [2]]
/// Output
/// [null, null, null, true, false, null, true, null, false]
///
/// Explanation
/// MyHashSet myHashSet = new MyHashSet();
/// myHashSet.add(1);      // set = [1]
/// myHashSet.add(2);      // set = [1, 2]
/// myHashSet.contains(1); // return True
/// myHashSet.contains(3); // return False, (not found)
/// myHashSet.add(2);      // set = [1, 2]
/// myHashSet.contains(2); // return True
/// myHashSet.remove(2);   // set = [1]
/// myHashSet.contains(2); // return False, (already removed)
///
///
/// Constraints:
/// * 0 <= key <= 10^6
/// * At most 10^4 calls will be made to add, remove, and contains.

pub struct MyHashSet {
    val: Vec<i32>,
}

impl MyHashSet {
    pub fn new() -> Self {
        Self { val: Vec::new() }
    }

    pub fn add(&mut self, key: i32) {
        if !self.val.contains(&key) {
            self.val.push(key);
        }
    }

    pub fn remove(&mut self, key: i32) {
        if self.val.contains(&key) {
            for i in 0..(self.val.len()) {
                if key == self.val[i] {
                    self.val.remove(i);
                    break;
                }
            }
        }
    }

    pub fn contains(&self, key: i32) -> bool {
        self.val.contains(&key)
    }
}

#[cfg(test)]
mod tests {
    use super::MyHashSet;

    #[test]
    fn test_my_hashset_1() {
        let mut obj = MyHashSet::new();

        obj.add(1);
        assert_eq!(obj.val, vec![1]);

        obj.add(2);
        assert_eq!(obj.val, vec![1, 2]);

        assert_eq!(obj.contains(1), true);
        assert_eq!(obj.contains(3), false);

        obj.add(2);
        assert_eq!(obj.val, vec![1, 2]);

        assert_eq!(obj.contains(2), true);

        obj.remove(2);
        assert_eq!(obj.val, vec![1]);

        assert_eq!(obj.contains(2), false);
    }

    #[test]
    fn test_my_hashset_2() {
        let mut obj = MyHashSet::new();

        obj.add(9);
        assert_eq!(obj.val, vec![9]);

        obj.remove(19);
        assert_eq!(obj.val, vec![9]);

        obj.add(14);
        assert_eq!(obj.val, vec![9, 14]);

        obj.remove(19);
        assert_eq!(obj.val, vec![9, 14]);

        obj.remove(9);
        assert_eq!(obj.val, vec![14]);

        obj.add(0);
        assert_eq!(obj.val, vec![14, 0]);

        obj.add(3);
        assert_eq!(obj.val, vec![14, 0, 3]);

        obj.add(4);
        assert_eq!(obj.val, vec![14, 0, 3, 4]);

        obj.add(0);
        assert_eq!(obj.val, vec![14, 0, 3, 4]);

        obj.remove(9);
        assert_eq!(obj.val, vec![14, 0, 3, 4]);
    }
}
