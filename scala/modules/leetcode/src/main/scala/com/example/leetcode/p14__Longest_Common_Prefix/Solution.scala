package com.example.leetcode.p14

/**
 * https://leetcode.com/problems/longest-common-prefix/
 *
 * Write a function to find the longest common prefix string amongst an array of strings.
 *
 * If there is no common prefix, return an empty string "".
 */

import scala.annotation.tailrec

object Solution {
  def longestCommonPrefix(strs: Array[String]): String = {
    val t = strs.tail

    @tailrec
    def f(prefix: String): String = {
      if (prefix.length == 0 || t.forall(_.startsWith(prefix))) prefix
      else f(prefix.dropRight(1))
    }

    f(strs.head)
  }
}
