/// 278. First Bad Version
/// Easy
///
/// You are a product manager and currently leading a team to develop a new product. Unfortunately, the latest version of your product fails the quality check. Since each version is developed based on the previous version, all the versions after a bad version are also bad.
///
/// Suppose you have n versions [1, 2, ..., n] and you want to find out the first bad one, which causes all the following ones to be bad.
///
/// You are given an API bool isBadVersion(version) which returns whether version is bad. Implement a function to find the first bad version. You should minimize the number of calls to the API.
///
/// Example 1:
///
/// Input: n = 5, bad = 4
/// Output: 4
/// Explanation:
/// call isBadVersion(3) -> false
/// call isBadVersion(5) -> true
/// call isBadVersion(4) -> true
/// Then 4 is the first bad version.
///
/// Example 2:
///
/// Input: n = 1, bad = 1
/// Output: 1
///
/// Constraints:
/// * 1 <= bad <= n <= 2^31 - 1

// The API isBadVersion is defined for you.
// isBadVersion(version:i32)-> bool;
// to call it use self.isBadVersion(version)

pub struct Solution {
    // for test
    bad: i32,
}

impl Solution {
    pub fn first_bad_version(&self, n: i32) -> i32 {
        if n == 1 {
            1
        } else {
            self.helper(1, n)
        }
    }

    fn helper(&self, first: i32, last: i32) -> i32 {
        if first == last {
            return first;
        }

        // Overflow countermeasures
        let half = first + (last - first) / 2;
        if self.isBadVersion(half) {
            self.helper(first, half)
        } else {
            self.helper(half + 1, last)
        }
    }

    // for test
    #[allow(non_snake_case)]
    fn isBadVersion(&self, version: i32) -> bool {
        if version >= self.bad {
            return true;
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_first_bad_version_1() {
        let n = 5;
        let solution = Solution { bad: 4 };
        let output = 4;
        assert_eq!(solution.first_bad_version(n), output);
    }

    #[test]
    fn test_first_bad_version_2() {
        let n = 1;
        let solution = Solution { bad: 1 };
        let output = 1;
        assert_eq!(solution.first_bad_version(n), output);
    }
}
