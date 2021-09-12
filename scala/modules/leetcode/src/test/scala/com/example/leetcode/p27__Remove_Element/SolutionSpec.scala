package com.example.leetcode.p27

class SolutionSpec extends munit.FunSuite {
  import Solution._

  def obtained(nums: Array[Int], expected: Array[Int]) = nums.dropRight(nums.length - expected.length)

  test("Input: nums = [3,2,2,3], val = 3 | Output: 2, nums = [2,2,_,_]") {
    val nums     = Array(3, 2, 2, 3)
    val expected = Array(2, 2)

    assertEquals(removeElement(nums, 3), 2)
    assertEquals(obtained(nums, expected).toSeq, expected.toSeq)
  }

  test("Input: nums = [0,1,2,2,3,0,4,2], val = 2 | Output: 5, nums = [0,1,4,0,3,_,_,_]") {
    val nums     = Array(0, 1, 2, 2, 3, 0, 4, 2)
    val expected = Array(0, 1, 4, 0, 3)

    assertEquals(removeElement(nums, 2), 5)
    assertEquals(obtained(nums, expected).toSet, expected.toSet)
  }
}
