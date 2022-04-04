// 412. Fizz Buzz
// Easy
//
// Given an integer n, return a string array answer (1-indexed) where:
//
// * answer[i] == "FizzBuzz" if i is divisible by 3 and 5.
// * answer[i] == "Fizz" if i is divisible by 3.
// * answer[i] == "Buzz" if i is divisible by 5.
// * answer[i] == i (as a string) if none of the above conditions are true.
//
//
// Example 1:
//
// Input: n = 3
// Output: ["1","2","Fizz"]
//
// Example 2:
//
// Input: n = 5
// Output: ["1","2","Fizz","4","Buzz"]
//
// Example 3:
//
// Input: n = 15
// Output: ["1","2","Fizz","4","Buzz","Fizz","7","8","Fizz","Buzz","11","Fizz","13","14","FizzBuzz"]
//
//
// Constraints:
// * 1 <= n <= 10^4

pub struct Solution {}
impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        let mut result = vec![];
        for i in 1..=n {
            result.push(Self::helper(i));
        }
        result
    }

    fn helper(n: i32) -> String {
        match n {
            _ if n % 15 == 0 => "FizzBuzz".to_string(),
            _ if n % 5 == 0 => "Buzz".to_string(),
            _ if n % 3 == 0 => "Fizz".to_string(),
            _ => n.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_fizz_buzz_1() {
        let n = 3;
        let output = vec!["1", "2", "Fizz"];
        assert_eq!(Solution::fizz_buzz(n), output);
    }

    #[test]
    fn test_fizz_buzz_2() {
        let n = 5;
        let output = vec!["1", "2", "Fizz", "4", "Buzz"];
        assert_eq!(Solution::fizz_buzz(n), output);
    }

    #[test]
    fn test_fizz_buzz_3() {
        let n = 15;
        let output = vec![
            "1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz", "11", "Fizz", "13",
            "14", "FizzBuzz",
        ];
        assert_eq!(Solution::fizz_buzz(n), output);
    }
}
