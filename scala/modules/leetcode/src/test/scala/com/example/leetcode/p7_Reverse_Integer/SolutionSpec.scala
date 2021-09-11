package com.example.leetcode.p7

class SolutionSpec extends munit.FunSuite {
  import Solution._

  test("Input: x = 123 | Output: 321") {
    assertEquals(reverse(123), 321)
  }

  test("Input: x = -123 | Output: -321") {
    assertEquals(reverse(-123), -321)
  }

  test("Input: x = 120 | Output: 21") {
    assertEquals(reverse(120), 21)
  }

  test("Input: x = 0 | Output: 0") {
    assertEquals(reverse(0), 0)
  }

  test("Input: x = Int.MaxValue | Output: 0") {
    assertEquals(reverse(Int.MaxValue), 0)
  }

  test("Input: x = Int.MinValue | Output: 0") {
    assertEquals(reverse(Int.MinValue), 0)
  }
}
