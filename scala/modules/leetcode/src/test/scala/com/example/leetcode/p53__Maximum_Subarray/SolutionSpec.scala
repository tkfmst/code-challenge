package com.example.leetcode.p53

class SolutionSpec extends munit.FunSuite {
  import Solution._

  test("""Input:  nums = [-2,1,-3,4,-1,2,1,-5,4] | Output: 6""") {
    assertEquals(maxSubArray(Array(-2, 1, -3, 4, -1, 2, 1, -5, 4)), 6)
  }

  test("""Input:  nums = [1] | Output: 1""") {
    assertEquals(maxSubArray(Array(1)), 1)
  }

  test("""Input:  nums = [5,4,-1,7,8] | Output: 23""") {
    assertEquals(maxSubArray(Array(5, 4, -1, 7, 8)), 23)
  }

  test("""Input:  nums = [-5,-4,-1,-7,-8] | Output: -1""") {
    assertEquals(maxSubArray(Array(-5, -4, -1, -7, -8)), -1)
  }
}
