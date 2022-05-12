// 47. Permutations II
// Medium
//
// Given a collection of numbers, nums, that might contain duplicates, return all possible unique permutations in any order.
//
// Example 1:
//
// Input: nums = [1,1,2]
// Output:
// [[1,1,2],
//  [1,2,1],
//  [2,1,1]]
//
// Example 2:
// Input: nums = [1,2,3]
// Output: [[1,2,3],[1,3,2],[2,1,3],[2,3,1],[3,1,2],[3,2,1]]
//
//
// Constraints:
// * 1 <= nums.length <= 8
// * -10 <= nums[i] <= 10

pub struct Solution {}
impl Solution {
    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = Vec::new();
        let mut cans: Vec<i32> = Vec::new();
        let mut nums = nums;
        let mut visited: Vec<bool> = vec![false; nums.len()];
        nums.sort_unstable();
        Solution::dfs_helper(nums, &mut cans, &mut res, &mut visited);
        res
    }

    fn dfs_helper(
        nums: Vec<i32>,
        cans: &mut Vec<i32>,
        res: &mut Vec<Vec<i32>>,
        visited: &mut Vec<bool>,
    ) {
        if cans.len() == nums.len() {
            res.push(cans.clone());
            return;
        }

        for (idx, &num) in nums.iter().enumerate() {
            if visited[idx] || idx > 0 && nums[idx] == nums[idx - 1] && !visited[idx - 1] {
                continue;
            }

            visited[idx] = true;
            cans.push(num);
            Solution::dfs_helper(nums.clone(), cans, res, visited);
            cans.pop();
            visited[idx] = false;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_permute_unique_1() {
        let nums = vec![1, 1, 2];
        let mut output = vec![[1, 1, 2], [1, 2, 1], [2, 1, 1]];
        output.sort();

        let mut actual = Solution::permute_unique(nums);
        actual.sort();

        assert_eq!(actual, output);
    }

    #[test]
    fn test_permute_unique_2() {
        let nums = vec![1, 2, 3];
        let mut output = vec![
            [1, 2, 3],
            [1, 3, 2],
            [2, 1, 3],
            [2, 3, 1],
            [3, 1, 2],
            [3, 2, 1],
        ];
        output.sort();

        let mut actual = Solution::permute_unique(nums);
        actual.sort();

        assert_eq!(actual, output);
    }
}
