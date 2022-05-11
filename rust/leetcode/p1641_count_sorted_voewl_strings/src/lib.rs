// 1641. Count Sorted Vowel Strings
// Medium
//
// Given an integer n, return the number of strings of length n that consist only of vowels (a, e, i, o, u) and are lexicographically sorted.
//
// A string s is lexicographically sorted if for all valid i, s[i] is the same as or comes before s[i+1] in the alphabet.
//
//
// Example 1:
//
// Input: n = 1
// Output: 5
// Explanation: The 5 sorted strings that consist of vowels only are ["a","e","i","o","u"].
//
// Example 2:
//
// Input: n = 2
// Output: 15
// Explanation: The 15 sorted strings that consist of vowels only are
// ["aa","ae","ai","ao","au","ee","ei","eo","eu","ii","io","iu","oo","ou","uu"].
// Note that "ea" is not a valid string since 'e' comes after 'a' in the alphabet.
//
// Example 3:
//
// Input: n = 33
// Output: 66045
//
//
// Constraints:
// * 1 <= n <= 50

pub struct Solution {}
impl Solution {
    /**
     * Characters are added from the last character to the previous one.
     * Think of it in dictionary form.
     *
     * Only `a` may be added before `a`.
     * Before `e`, `a` and `e` can be added.
     * Before `i`, `a`, `e`, and `i` can be added.
     * Before `o`, `a`, `e`, `i`, and `o` can be added.
     * Before `u`, `a`, `e`, `i`, `o`, `u` can be added.
     *
     * This is repeated `n` times. n` times.
     */
    pub fn count_vowel_strings(mut n: i32) -> i32 {
        let mut a = 1;
        let mut e = 1;
        let mut i = 1;
        let mut o = 1;
        let mut u = 1;

        while n > 1 {
            a = a + e + i + o + u;
            e = e + i + o + u;
            i = i + o + u;
            o = o + u;
            u = u;

            n -= 1;
        }

        a + e + i + o + u
    }
}
#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_count_vowel_strings_1() {
        let n = 1;
        let output = 5;
        assert_eq!(Solution::count_vowel_strings(n), output);
    }

    #[test]
    fn test_count_vowel_strings_2() {
        let n = 2;
        let output = 15;
        assert_eq!(Solution::count_vowel_strings(n), output);
    }

    #[test]
    fn test_count_vowel_strings_3() {
        let n = 33;
        let output = 66045;
        assert_eq!(Solution::count_vowel_strings(n), output);
    }
}
