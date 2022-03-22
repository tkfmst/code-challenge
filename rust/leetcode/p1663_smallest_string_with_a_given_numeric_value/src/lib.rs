// 1663. Smallest String With A Given Numeric Value
// Medium
//
// The numeric value of a lowercase character is defined as its position (1-indexed) in the alphabet, so the numeric value of a is 1, the numeric value of b is 2, the numeric value of c is 3, and so on.
//
// The numeric value of a string consisting of lowercase characters is defined as the sum of its characters' numeric values. For example, the numeric value of the string "abe" is equal to 1 + 2 + 5 = 8.
//
// You are given two integers n and k. Return the lexicographically smallest string with length equal to n and numeric value equal to k.
//
// Note that a string x is lexicographically smaller than string y if x comes before y in dictionary order, that is, either x is a prefix of y, or if i is the first position such that x[i] != y[i], then x[i] comes before y[i] in alphabetic order.
//
//
// Example 1:
//
// Input: n = 3, k = 27
// Output: "aay"
// Explanation: The numeric value of the string is 1 + 1 + 25 = 27, and it is the smallest string with such a value and length equal to 3.
//
// Example 2:
//
// Input: n = 5, k = 73
// Output: "aaszz"
//
//
// Constraints:
//
// * 1 <= n <= 10^5
// * n <= k <= 26 * n

pub struct Solution {}
impl Solution {
    pub fn get_smallest_string(n: i32, k: i32) -> String {
        let mut result: Vec<u8> = std::iter::repeat(b'a').take(n as usize).collect();
        let mut remaining = k - n;

        let z = (b'z' - b'a') as i32;
        for i in 0..n {
            let idx = (n - i - 1) as usize;
            if remaining > z {
                remaining -= z;
                result[idx] = b'z';
            } else {
                result[idx] = remaining as u8 + b'a';
                break;
            }
        }
        std::str::from_utf8(result.as_slice()).unwrap().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_get_smallest_string_1() {
        let n = 3;
        let k = 27;
        let output = "aay".to_string();
        assert_eq!(Solution::get_smallest_string(n, k), output);
    }

    #[test]
    fn test_get_smallest_string_2() {
        let n = 5;
        let k = 73;
        let output = "aaszz".to_string();
        assert_eq!(Solution::get_smallest_string(n, k), output);
    }
}
