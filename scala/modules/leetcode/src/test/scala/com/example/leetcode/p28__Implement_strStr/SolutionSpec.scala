package com.example.leetcode.p28

class SolutionSpec extends munit.FunSuite {
  import Solution._

  test("""Input:  haystack = "hello", needle = "ll" | Output: 22""") {
    assertEquals(strStr("hello", "ll"), 2)
  }

  test("""Input:  haystack = "aaaaa", needle = "bba" | Output: -1""") {
    assertEquals(strStr("aaaaaa", "bba"), -1)
  }

  test("""Input:  haystack = "", needle = "" | Output: 0""") {
    assertEquals(strStr("", ""), 0)
  }
}
