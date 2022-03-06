/// 303. Range Sum Query - Immutable
/// Easy
///
/// Given an integer array nums, handle multiple queries of the following type:
/// 1. Calculate the sum of the elements of nums between indices left and right inclusive where left <= right.
///
/// Implement the NumArray class:
/// * NumArray(int[] nums) Initializes the object with the integer array nums.
/// * int sumRange(int left, int right) Returns the sum of the elements of nums between indices left and right inclusive (i.e. nums[left] + nums[left + 1] + ... + nums[right]).
///
/// Example 1:
///
/// Input
/// ["NumArray", "sumRange", "sumRange", "sumRange"]
/// [[[-2, 0, 3, -5, 2, -1]], [0, 2], [2, 5], [0, 5]]
/// Output
/// [null, 1, -1, -3]
///
/// Explanation
/// NumArray numArray = new NumArray([-2, 0, 3, -5, 2, -1]);
/// numArray.sumRange(0, 2); // return (-2) + 0 + 3 = 1
/// numArray.sumRange(2, 5); // return 3 + (-5) + 2 + (-1) = -1
/// numArray.sumRange(0, 5); // return (-2) + 0 + 3 + (-5) + 2 + (-1) = -3
///
///
/// Constraints:
///
/// * 1 <= nums.length <= 104
/// * -105 <= nums[i] <= 105
/// * 0 <= left <= right < nums.length
/// * At most 104 calls will be made to sumRange.

pub struct NumArray {
    arr: Vec<i32>,
}

impl NumArray {
    pub fn new(nums: Vec<i32>) -> Self {
        Self { arr: nums }
    }

    pub fn sum_range(&self, left: i32, right: i32) -> i32 {
        self.arr[(left as usize)..((right + 1) as usize)]
            .iter()
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::NumArray;

    #[test]
    fn test_num_array_1() {
        let nums = vec![-2, 0, 3, -5, 2, -1];

        let num_array = NumArray::new(nums);

        assert_eq!(num_array.sum_range(0, 2), 1);
        assert_eq!(num_array.sum_range(2, 5), -1);
        assert_eq!(num_array.sum_range(0, 5), -3);
    }
}
