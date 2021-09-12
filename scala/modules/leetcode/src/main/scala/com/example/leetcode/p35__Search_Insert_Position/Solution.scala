package com.example.leetcode.p35

/**
 * https://leetcode.com/problems/search-insert-position/
 *
 * Given a sorted array of distinct integers and a target value, return the index if the target is found. If not, return
 * the index where it would be if it were inserted in order.
 *
 * You must write an algorithm with O(log n) runtime complexity.
 */

object Solution {
  // def searchInsert(nums: Array[Int], target: Int): Int = {
  //   nums.indexOf(target) match {
  //     case -1 => (nums :+ target).sorted.indexOf(target)
  //     case n  => n
  //   }
  //

  def searchInsert(nums: Array[Int], target: Int): Int = {
    @scala.annotation.tailrec
    def f(idx: Int): Int =
      if (idx >= nums.length) idx
      else if (nums(idx) >= target) idx
      else f(idx + 1)

    f(0)
  }
}
