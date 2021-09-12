package com.example.leetcode.p27

/**
 * https://leetcode.com/problems/remove-element/
 *
 * Given an integer array nums and an integer val, remove all occurrences of val in nums in-place. The relative order of
 * the elements may be changed.
 *
 * Since it is impossible to change the length of the array in some languages, you must instead have the result be
 * placed in the first part of the array nums. More formally, if there are k elements after removing the duplicates,
 * then the first k elements of nums should hold the final result. It does not matter what you leave beyond the first k
 * elements.
 *
 * Return k after placing the final result in the first k slots of nums.
 *
 * Do not allocate extra space for another array. You must do this by modifying the input array in-place with O(1) extra
 * memory.
 *
 * Custom Judge:
 *
 * The judge will test your solution with the following code:
 *
 * {{{
 * int[] nums = [...]; // Input array
 * int val = ...; // Value to remove
 * int[] expectedNums = [...]; // The expected answer with correct length.
 *                             // It is sorted with no values equaling val.
 *
 * int k = removeElement(nums, val); // Calls your implementation
 *
 * assert k == expectedNums.length;
 * sort(nums, 0, k); // Sort the first k elements of nums
 * for (int i = 0; i < actualLength; i++) {
 *     assert nums[i] == expectedNums[i];
 * }
 * }}}
 * If all assertions pass, then your solution will be accepted.
 */

object Solution {
  def removeElement(nums: Array[Int], `val`: Int): Int = {
    @scala.annotation.tailrec
    def f(idx: Int, nxt: Int): Int = {
      if (nums.length == nxt) nums(idx) match {
        case `val` => idx
        case _     => idx + 1
      }
      else
        (nums(idx), nums(nxt)) match {
          case (`val`, `val`) =>
            f(idx, nxt + 1)
          case (`val`, n)     =>
            nums(idx) = n
            nums(nxt) = `val`
            f(idx + 1, nxt + 1)
          case _              =>
            f(idx + 1, nxt + 1)
        }
    }

    if (nums.isEmpty) 0
    else f(0, 1)
  }
}
