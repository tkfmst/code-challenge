package com.example.leetcode.p53

/**
 * https://leetcode.com/problems/maximum-subarray/
 *
 * Given an integer array nums, find the contiguous subarray (containing at least one number) which has the largest sum
 * and return its sum.
 *
 * A subarray is a contiguous part of an array.
 *
 * Constraints:
 *
 * 1 <= nums.length <= 3 * 10^4
 * -10^5 <= nums[i] <= 10^5
 */

object Solution {
  def maxSubArray(nums: Array[Int]): Int = {
    nums
      .foldLeft((0, Int.MinValue)) {
        case ((sum, max), n) => {
          val sum1 = Math.max(sum, 0) + n
          (sum1, Math.max(max, sum1))
        }
      }
      ._2
  }
}
