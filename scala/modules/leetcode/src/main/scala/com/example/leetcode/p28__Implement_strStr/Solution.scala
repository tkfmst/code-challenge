package com.example.leetcode.p28

/**
 * https://leetcode.com/problems/implement-strstr/
 *
 * Implement strStr().
 *
 * Return the index of the first occurrence of needle in haystack, or -1 if needle is not part of haystack.
 *
 * Clarification:
 *
 * What should we return when needle is an empty string? This is a great question to ask during an interview.
 *
 * For the purpose of this problem, we will return 0 when needle is an empty string. This is consistent to C's strstr()
 * and Java's indexOf().
 */

object Solution {

  // def strStr(haystack: String, needle: String): Int = {
  //   if (needle == "") {
  //     0
  //   }
  //   else {
  //     haystack
  //       .sliding(needle.length)
  //       .zipWithIndex
  //       .collectFirst({ case (`needle`, y) => y })
  //       .getOrElse(-1)
  //   }
  // }

  def strStr(haystack: String, needle: String): Int = {
    def startWith(str: String): Boolean =
      (str.isEmpty, needle.isEmpty) match {
        case (true, true)   => true
        case (true, false)  => false
        case (false, true)  => true
        case (false, false) =>
          str.dropRight(str.length - needle.length) == needle
      }

    def f(haystack: String, idx: Int): Int =
      (haystack.isEmpty, needle.isEmpty) match {
        case (true, true)   => idx
        case (true, false)  => -1
        case (false, true)  => 0
        case (false, false) =>
          (haystack.head == needle.head, startWith(haystack)) match {
            case (true, true) => idx
            case _            => f(haystack.tail, idx + 1)
          }
      }

    f(haystack, 0)
  }
}
