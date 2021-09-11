package com.example.leetcode.p20

/**
 * https://leetcode.com/problems/valid-parentheses/
 *
 * Given a string s containing just the characters '(', ')', '{', '}', '[' and ']', determine if the input string is
 * valid.
 *
 * An input string is valid if:
 *
 * Open brackets must be closed by the same type of brackets. Open brackets must be closed in the correct order.
 */

import scala.annotation.tailrec

object Solution {
  private[this] val pairs   = Map(
    '{' -> '}',
    '(' -> ')',
    '[' -> ']',
  )
  private[this] val openers = pairs.keys

  def isValid(s: String): Boolean =
    if (s.length < 2) false
    else {
      val stack = scala.collection.mutable.Stack.empty[Char]

      @tailrec
      def f(fst: Char, rem: String): Boolean = {
        if (rem.isEmpty) false
        else {
          val snd = rem.head
          val nxt = rem.tail

          pairs.get(fst) match {
            case Some(close) if close == snd   =>
              if (stack.isEmpty) nxt.length match {
                case 0 => true
                case 1 => false
                case _ => f(nxt.head, nxt.tail)
              }
              else f(stack.pop(), nxt)
            case _ if openers.exists(_ == snd) =>
              stack.push(fst)
              f(snd, nxt)
            case _                             => false
          }
        }
      }

      f(s.head, s.tail)
    }
}
