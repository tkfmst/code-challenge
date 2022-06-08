/// 551. Student Attendance Record I
/// Easy
///
/// You are given a string s representing an attendance record for a student where each character signifies whether the student was absent, late, or present on that day. The record only contains the following three characters:
///
/// 'A': Absent.
/// 'L': Late.
/// 'P': Present.
///
/// The student is eligible for an attendance award if they meet both of the following criteria:
///
/// The student was absent ('A') for strictly fewer than 2 days total.
/// The student was never late ('L') for 3 or more consecutive days.
///
/// Return true if the student is eligible for an attendance award, or false otherwise.
///
///
/// Example 1:
///
/// Input: s = "PPALLP"
/// Output: true
/// Explanation: The student has fewer than 2 absences and was never late 3 or more consecutive days.
///
/// Example 2:
///
/// Input: s = "PPALLL"
/// Output: false
/// Explanation: The student was late 3 consecutive days in the last 3 days, so is not eligible for the award.
///
///
/// Constraints:
/// * 1 <= s.length <= 1000
/// * s[i] is either 'A', 'L', or 'P'.

pub struct Solution;
impl Solution {
    pub fn check_record(s: String) -> bool {
        let mut absent_cnt = 0;
        let mut late_cnt = 0;

        let absent = b'A';
        let late = b'L';

        for &b in s.as_bytes() {
            if b == absent {
                absent_cnt += 1;
                if absent_cnt > 1 {
                    return false;
                }
                late_cnt = 0;
            } else if b == late {
                late_cnt += 1;
                if late_cnt > 2 {
                    return false;
                }
            } else {
                late_cnt = 0;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_check_record_1() {
        let s = "PPALLP".to_string();
        let output = true;
        assert_eq!(Solution::check_record(s), output);
    }

    #[test]
    fn test_check_record_2() {
        let s = "PPALLL".to_string();
        let output = false;
        assert_eq!(Solution::check_record(s), output);
    }

    #[test]
    fn test_check_record_3() {
        let s = "LALL".to_string();
        let output = true;
        assert_eq!(Solution::check_record(s), output);
    }
}
