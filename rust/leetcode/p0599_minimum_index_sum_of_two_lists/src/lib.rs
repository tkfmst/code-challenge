/// 599. Minimum Index Sum of Two Lists
/// Easy
///
/// Suppose Andy and Doris want to choose a restaurant for dinner, and they both have a list of favorite restaurants represented by strings.
///
/// You need to help them find out their common interest with the least list index sum. If there is a choice tie between answers, output all of them with no order requirement. You could assume there always exists an answer.
///
///  
///
/// Example 1:
///
/// Input: list1 = ["Shogun","Tapioca Express","Burger King","KFC"], list2 = ["Piatti","The Grill at Torrey Pines","Hungry Hunter Steakhouse","Shogun"]
/// Output: ["Shogun"]
/// Explanation: The only restaurant they both like is "Shogun".
///
/// Example 2:
///
/// Input: list1 = ["Shogun","Tapioca Express","Burger King","KFC"], list2 = ["KFC","Shogun","Burger King"]
/// Output: ["Shogun"]
/// Explanation: The restaurant they both like and have the least index sum is "Shogun" with index sum 1 (0+1).
///
///  
///
/// Constraints:
///
/// * 1 <= list1.length, list2.length <= 1000
/// * 1 <= list1[i].length, list2[i].length <= 30
/// * list1[i] and list2[i] consist of spaces ' ' and English letters.
/// * All the stings of list1 are unique.
/// * All the stings of list2 are unique.

pub struct Solution {}

impl Solution {
    pub fn find_restaurant(list1: Vec<String>, list2: Vec<String>) -> Vec<String> {
        use std::cmp::Ordering;
        use std::collections::HashMap;

        let map = list1.iter().zip(0..).collect::<HashMap<_, usize>>();
        let mut res = vec![];
        let mut min_sum = usize::MAX;

        for (i2, s2) in list2.into_iter().enumerate() {
            if let Some(&i1) = map.get(&s2) {
                let new_sum = i1 + i2;
                match new_sum.cmp(&min_sum) {
                    Ordering::Less => {
                        min_sum = new_sum;
                        res = vec![s2];
                    }
                    Ordering::Equal => res.push(s2),
                    _ => (),
                }
            }
            if i2 > min_sum {
                break;
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_find_restaurant_1() {
        let list1 = vec![
            "Shogun".to_string(),
            "Tapioca Express".to_string(),
            "Burger King".to_string(),
            "KFC".to_string(),
        ];
        let list2 = vec![
            "Piatti".to_string(),
            "The Grill at Torrey Pines".to_string(),
            "Hungry Hunter Steakhouse".to_string(),
            "Shogun".to_string(),
        ];
        let output = vec!["Shogun".to_string()];
        assert_eq!(Solution::find_restaurant(list1, list2), output);
    }

    #[test]
    fn test_find_restaurant_2() {
        let list1 = vec![
            "Shogun".to_string(),
            "Tapioca Express".to_string(),
            "Burger King".to_string(),
            "KFC".to_string(),
        ];
        let list2 = vec![
            "KFC".to_string(),
            "Shogun".to_string(),
            "Burger King".to_string(),
        ];
        let output = vec!["Shogun".to_string()];
        assert_eq!(Solution::find_restaurant(list1, list2), output);
    }
}
