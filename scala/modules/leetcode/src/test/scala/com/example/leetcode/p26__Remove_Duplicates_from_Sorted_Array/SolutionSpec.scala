package com.example.leetcode.p26

class SolutionSpec extends munit.FunSuite {
  import Solution._

  def obtained(nums: Array[Int], expected: Array[Int]) = nums.dropRight(nums.length - expected.length)

  test("Input: nums = [1,2,3] | Output: 2, nums = [1,2,_]") {
    val nums     = Array(1, 1, 2)
    val expected = Array(1, 2)
    assertEquals(
      removeDuplicates(nums),
      2
    )
    assertEquals(obtained(nums, expected).toSeq, expected.toSeq)

  }
  test("Input: nums = [0,0,1,1,1,2,2,3,3,4] | Output: 5, nums = [0,1,2,3,4,_,_,_,_,_]") {
    val nums     = Array(0, 0, 1, 1, 1, 2, 2, 3, 3, 4)
    val expected = Array(0, 1, 2, 3, 4)
    assertEquals(
      removeDuplicates(nums),
      5
    )
    assertEquals(obtained(nums, expected).toSeq, expected.toSeq)
  }
}
