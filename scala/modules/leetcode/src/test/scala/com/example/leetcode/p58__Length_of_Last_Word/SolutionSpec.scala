package com.example.leetcode.p58

class SolutionSpec extends munit.FunSuite {
  import Solution._

  test("""Input:  s = "Hello World" | Output: 5""") {
    assertEquals(lengthOfLastWord("Hello World"), 5)
  }

  test("""Input:  s = "   fly me   to   the moon  " | Output: 4""") {
    assertEquals(lengthOfLastWord("   fly me   to   the moon  "), 4)
  }

  test("""Input:  s = "luffy is still joyboy" | Output: 6""") {
    assertEquals(lengthOfLastWord("luffy is still joyboy"), 6)
  }
}
