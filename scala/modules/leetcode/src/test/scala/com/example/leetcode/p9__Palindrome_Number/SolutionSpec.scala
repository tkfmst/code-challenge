package com.example.leetcode.p9

class SolutionSpec extends munit.FunSuite {
  import Solution._

  test("Input: x = 121 | Output: true") {
    assertEquals(isPalindrome(121), true)
  }

  test("Input: x = -121 | Output: false") {
    assertEquals(isPalindrome(-121), false)
  }

  test("Input: x = 10 | Output: false") {
    assertEquals(isPalindrome(10), false)
  }

  test("Input: x = -101 | Output: false") {
    assertEquals(isPalindrome(-101), false)
  }
}
