package com.example.leetcode.p13

class SolutionSpec extends munit.FunSuite {
  import Solution._

  test("III == 3") {
    assertEquals(romanToInt("III"), 3)
  }

  test("IV == 4") {
    assertEquals(romanToInt("IV"), 4)
  }

  test("IX == 9") {
    assertEquals(romanToInt("IX"), 9)
  }

  test("LVIII == 58") {
    assertEquals(romanToInt("LVIII"), 58)
  }

  test("MCMXCIV == 1994") {
    assertEquals(romanToInt("MCMXCIV"), 1994)
  }
}
