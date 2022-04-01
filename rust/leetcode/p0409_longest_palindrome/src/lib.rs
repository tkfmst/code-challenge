/// 409. Longest Palindrome
/// Easy
///
/// Given a string s which consists of lowercase or uppercase letters, return the length of the longest palindrome that can be built with those letters.
///
/// Letters are case sensitive, for example, "Aa" is not considered a palindrome here.
///
///
/// Example 1:
///
/// Input: s = "abccccdd"
/// Output: 7
/// Explanation:
/// One longest palindrome that can be built is "dccaccd", whose length is 7.
///
/// Example 2:
///
/// Input: s = "a"
/// Output: 1
///
/// Example 3:
///
/// Input: s = "bb"
/// Output: 2
///
///
/// Constraints:
///
/// * 1 <= s.length <= 2000
/// * s consists of lowercase and/or uppercase English letters only.

pub struct Solution {}
impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        use std::collections::HashSet;

        let mut set = HashSet::with_capacity(52);
        let mut cnt = 0;

        for c in s.chars() {
            if set.contains(&c) {
                set.remove(&c);
                cnt += 2;
            } else {
                set.insert(c);
            }
        }

        if set.is_empty() {
            cnt
        } else {
            cnt + 1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_longest_palindrome_1() {
        let s = "abccccdd".to_string();
        let output = 7;
        assert_eq!(Solution::longest_palindrome(s), output);
    }

    #[test]
    fn test_longest_palindrome_2() {
        let s = "a".to_string();
        let output = 1;
        assert_eq!(Solution::longest_palindrome(s), output);
    }

    #[test]
    fn test_longest_palindrome_3() {
        let s = "bb".to_string();
        let output = 2;
        assert_eq!(Solution::longest_palindrome(s), output);
    }

    #[test]
    fn test_longest_palindrome_4() {
        let s = "civilwartestingwhetherthatnaptionoranynartionsoconceivedandsodedicatedcanlongendureWeareqmetonagreatbattlefiemldoftzhatwarWehavecometodedicpateaportionofthatfieldasafinalrestingplaceforthosewhoheregavetheirlivesthatthatnationmightliveItisaltogetherfangandproperthatweshoulddothisButinalargersensewecannotdedicatewecannotconsecratewecannothallowthisgroundThebravelmenlivinganddeadwhostruggledherehaveconsecrateditfaraboveourpoorponwertoaddordetractTgheworldadswfilllittlenotlenorlongrememberwhatwesayherebutitcanneverforgetwhattheydidhereItisforusthelivingrathertobededicatedheretotheulnfinishedworkwhichtheywhofoughtherehavethusfarsonoblyadvancedItisratherforustobeherededicatedtothegreattdafskremainingbeforeusthatfromthesehonoreddeadwetakeincreaseddevotiontothatcauseforwhichtheygavethelastpfullmeasureofdevotionthatweherehighlyresolvethatthesedeadshallnothavediedinvainthatthisnationunsderGodshallhaveanewbirthoffreedomandthatgovernmentofthepeoplebythepeopleforthepeopleshallnotperishfromtheearth".to_string();
        let output = 983;
        assert_eq!(Solution::longest_palindrome(s), output);
    }
}
