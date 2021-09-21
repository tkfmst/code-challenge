package com.example.leetcode.p58

/**
 * https://leetcode.com/problems/length-of-last-word/
 *
 * Given a string s consisting of some words separated by some number of spaces, return the length of the last word in
 * the string.
 *
 * A word is a maximal substring consisting of non-space characters only.
 */

object Solution {
  def lengthOfLastWord(s: String): Int = {
    val ss = s.trim
    ss.trim.splitAt(ss.trim.lastIndexOf(" ") + 1)._2.size
  }
}
