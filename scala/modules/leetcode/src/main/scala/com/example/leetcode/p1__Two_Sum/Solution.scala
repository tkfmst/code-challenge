package com.example.leetcode.p1

/**
 * https://leetcode.com/problems/two-sum/
 *
 * Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to
 * target.
 *
 * You may assume that each input would have exactly one solution, and you may not use the same element twice.
 *
 * You can return the answer in any order.
 */

object Solution {
  def twoSum(nums: Array[Int], target: Int): Array[Int] = {
    val map = nums.indices.foldLeft(scala.collection.mutable.Map.empty[Int, Int])((acc, i) => {
      acc += (nums(i) -> i); acc
    })
    val idx = nums.indices.indexWhere { i =>
      val v = target - nums(i)
      if (map.contains(v) && i != map(v)) true else false
    }
    Array(idx, map(target - nums(idx)))
  }
}
