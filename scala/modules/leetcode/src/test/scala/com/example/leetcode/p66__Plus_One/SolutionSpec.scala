package com.example.leetcode.p66

class SolutionSpec extends munit.FunSuite {
  import Solution._

  test("""Input:  digits = [1,2,3] | Output: [1,2,4]""") {
    assertEquals(plusOne(Array(1, 2, 3)).toSeq, Array(1, 2, 4).toSeq)
  }

  test("""Input:  digits = [4,3,2,1] | Output: [4,3,2,2]""") {
    assertEquals(plusOne(Array(4, 3, 2, 1)).toSeq, Array(4, 3, 2, 2).toSeq)
  }

  test("""Input:  digits = [0] | Output: [1]""") {
    assertEquals(plusOne(Array(0)).toSeq, Array(1).toSeq)
  }

  test("""Input:  digits = [9] | Output: [1, 0]""") {
    assertEquals(plusOne(Array(9)).toSeq, Array(1, 0).toSeq)
  }
}
