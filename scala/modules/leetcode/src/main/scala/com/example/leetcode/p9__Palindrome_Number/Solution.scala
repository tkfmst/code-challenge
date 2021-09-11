package com.example.leetcode.p9

/**
 * https://leetcode.com/problems/palindrome-number/
 *
 * Given an integer x, return true if x is palindrome integer.
 *
 * An integer is a palindrome when it reads the same backward as forward. For example, 121 is palindrome while 123 is
 * not.
 */

object Solution {
  def isPalindrome(x: Int): Boolean = {
    if (x < 0) false
    else x.toString.reverse == x.toString
  }
}
