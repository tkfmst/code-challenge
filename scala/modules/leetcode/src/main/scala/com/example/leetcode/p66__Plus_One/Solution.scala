package com.example.leetcode.p66

/**
 * https://leetcode.com/problems/plus-one/
 *
 * You are given a large integer represented as an integer array digits, where each digits[i] is the ith digit of the
 * integer. The digits are ordered from most significant to least significant in left-to-right order. The large integer
 * does not contain any leading 0's.
 *
 * Increment the large integer by one and return the resulting array of digits.
 */

object Solution {
  def plusOne(digits: Array[Int]): Array[Int] =
    (BigInt(digits.mkString("")) + 1).toString.sliding(1).toArray.map(_.toInt)
}
