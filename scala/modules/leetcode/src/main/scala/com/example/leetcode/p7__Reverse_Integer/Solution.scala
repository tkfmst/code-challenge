package com.example.leetcode.p7

/**
 * https://leetcode.com/problems/reverse-integer/
 *
 * Given a signed 32-bit integer x, return x with its digits reversed. If reversing x causes the value to go outside the
 * signed 32-bit integer range [-231, 231 - 1], then return 0.
 *
 * Assume the environment does not allow you to store 64-bit integers (signed or unsigned).
 */

import scala.annotation.tailrec

object Solution {
  def reverse(x: Int): Int = {
    @tailrec
    def f(x: Int, output: Int = 0): Int =
      if (output < Int.MinValue / 10 || output > Int.MaxValue / 10) 0
      else if (x / 10 == 0) (output * 10) + (x % 10)
      else f(x / 10, (output * 10) + x % 10)

    f(x)
  }
}
