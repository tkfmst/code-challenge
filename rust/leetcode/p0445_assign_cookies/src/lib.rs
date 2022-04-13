/// 455. Assign Cookies
/// Easy
///
/// Assume you are an awesome parent and want to give your children some cookies. But, you should give each child at most one cookie.
///
/// Each child i has a greed factor g[i], which is the minimum size of a cookie that the child will be content with; and each cookie j has a size s[j]. If s[j] >= g[i], we can assign the cookie j to the child i, and the child i will be content. Your goal is to maximize the number of your content children and output the maximum number.
///
///
/// Example 1:
///
/// Input: g = [1,2,3], s = [1,1]
/// Output: 1
/// Explanation: You have 3 children and 2 cookies. The greed factors of 3 children are 1, 2, 3.
/// And even though you have 2 cookies, since their size is both 1, you could only make the child whose greed factor is 1 content.
/// You need to output 1.
///
/// Example 2:
///
/// Input: g = [1,2], s = [1,2,3]
/// Output: 2
/// Explanation: You have 2 children and 3 cookies. The greed factors of 2 children are 1, 2.
/// You have 3 cookies and their sizes are big enough to gratify all of the children,
/// You need to output 2.
///
///
/// Constraints:
/// * 1 <= g.length <= 3 * 10^4
/// * 0 <= s.length <= 3 * 10^4
/// * 1 <= g[i], s[j] <= 2^33 - 1

pub struct Solution {}
impl Solution {
    pub fn find_content_children(g: Vec<i32>, s: Vec<i32>) -> i32 {
        // answer2
        // Runtime: 8ms
        // Memory: 2.5MB
        let mut g_mut = g;
        let mut s_mut = s;
        g_mut.sort();
        s_mut.sort();

        let mut i = 0;
        let mut j = 0;
        let mut cnt = 0;

        while i < g_mut.len() && j < s_mut.len() {
            if g_mut[i] <= s_mut[j] {
                cnt += 1;
                i += 1;
            }
            j += 1;
        }

        cnt

        // // answer 1
        // // Runtime: 8ms
        // // Memory: 2.4MB
        // let mut g_mut = g;
        // let mut s_mut = s;
        // g_mut.sort();
        // s_mut.sort();
        //
        // let mut it_g = g_mut.iter();
        // let mut it_s = s_mut.iter();
        //
        // let mut cnt = 0;
        //
        // while let Some(gi) = it_g.next() {
        //     while let Some(sj) = it_s.next() {
        //         if gi <= sj {
        //             cnt += 1;
        //             break;
        //         }
        //     }
        // }
        //
        // cnt
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_find_content_children_1() {
        let g = vec![1, 2, 3];
        let s = vec![1, 1];
        let output = 1;
        assert_eq!(Solution::find_content_children(g, s), output);
    }

    #[test]
    fn test_find_content_children_2() {
        let g = vec![1, 2];
        let s = vec![1, 2, 3];
        let output = 2;
        assert_eq!(Solution::find_content_children(g, s), output);
    }

    #[test]
    fn test_find_content_children_a() {
        let g = vec![2, 2];
        let s = vec![1, 2, 3];
        let output = 2;
        assert_eq!(Solution::find_content_children(g, s), output);
    }

    #[test]
    fn test_find_content_children_b() {
        let g = vec![2];
        let s = vec![1];
        let output = 0;
        assert_eq!(Solution::find_content_children(g, s), output);
    }
}
