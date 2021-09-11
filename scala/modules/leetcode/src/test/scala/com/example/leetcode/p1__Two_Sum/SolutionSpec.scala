package com.example.leetcode.p1

class SolutionSpec extends munit.FunSuite {
  import Solution._

  test("Input: nums = [2,7,11,15], target = 9 | Output: [0,1]") {
    assertEquals(twoSum(Array(2, 7, 11, 15), 9).toSeq, Array(0, 1).toSeq)
  }

  test("Input: nums = [3,2,4], target = 6 | Output: [1,2]") {
    assertEquals(twoSum(Array(3, 2, 4), 6).toSeq, Array(1, 2).toSeq)
  }

  test("Input: nums = [3,3], target = 6 | Output: [0,1]") {
    assertEquals(twoSum(Array(3, 3), 6).toSeq, Array(0, 1).toSeq)
  }
}
