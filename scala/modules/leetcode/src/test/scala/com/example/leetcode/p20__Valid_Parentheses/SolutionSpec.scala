package com.example.leetcode.p20

class SolutionSpec extends munit.FunSuite {
  import Solution._

  test("""Input: s = "()" | Output: true""") {
    assertEquals(isValid("()"), true)
  }

  test("""Input: s = "()[]{}" | Output: true""") {
    assertEquals(isValid("()[]{}"), true)
  }

  test("""Input: s = "(]" | Output: false""") {
    assertEquals(isValid("(]"), false)
  }

  test("""Input: s = "([)]" | Output: false""") {
    assertEquals(isValid("([)]"), false)
  }

  test("""Input: s = "{[]}" | Output: true""") {
    assertEquals(isValid("{[]}"), true)
  }

  test("""Input: s = "[" | Output: false""") {
    assertEquals(isValid("["), false)
  }

  test("""Input: s = "){" | Output: false""") {
    assertEquals(isValid("){"), false)
  }

  test("""Input: s = "[([]])" | Output: false""") {
    assertEquals(isValid("[([]])"), false)
  }

  test("""Input: s = "(([]){})" | Output: false""") {
    assertEquals(isValid("(([]){})"), true)
  }
}
