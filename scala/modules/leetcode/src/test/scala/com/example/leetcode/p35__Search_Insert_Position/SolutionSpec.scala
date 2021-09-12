package com.example.leetcode.p35

class SolutionSpec extends munit.FunSuite {
  import Solution._

  test("""Input:  nums = [1,3,5,6], target = 5 | Output: 2""") {
    assertEquals(searchInsert(Array(1, 3, 5, 6), 5), 2)
  }

  test("""Input:  nums = [1,3,5,6], target = 2 | Output: 1""") {
    assertEquals(searchInsert(Array(1, 3, 5, 6), 2), 1)
  }

  test("""Input:  nums = [1,3,5,6], target = 7 | Output: 4""") {
    assertEquals(searchInsert(Array(1, 3, 5, 6), 7), 4)
  }

  test("""Input:  nums = [1,3,5,6], target = 0 | Output: 0""") {
    assertEquals(searchInsert(Array(1, 3, 5, 6), 0), 0)
  }

  test("""Input:  nums = [1], target = 0 | Output: 0""") {
    assertEquals(searchInsert(Array(1), 0), 0)
  }
}
