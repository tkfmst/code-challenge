package com.example.leetcode.p14

class SolutionSpec extends munit.FunSuite {
  import Solution._

  test("""Input: strs = ["flower","flow","flight"] | Output: "fl" """) {
    assertEquals(longestCommonPrefix(Array("flower", "flow", "flight")), "fl")
  }

  test("""Input: strs = ["dog","racecar","car"] | Output: "" """) {
    assertEquals(longestCommonPrefix(Array("dog", "racecar", "car")), "")
  }
}
