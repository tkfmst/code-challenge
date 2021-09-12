package com.example.leetcode.p100

/**
 * https://leetcode.com/problems/same-tree/
 *
 * Given the roots of two binary trees p and q, write a function to check if they are the same or not.
 *
 * Two binary trees are considered the same if they are structurally identical, and the nodes have the same value.
 */

// Definition for a binary tree node.
@SuppressWarnings(Array("org.wartremover.warts.Var", "org.wartremover.warts.Null"))
class TreeNode(val value: Int = 0, val left: TreeNode = null, val right: TreeNode = null)

object Solution {
  @SuppressWarnings(Array("org.wartremover.warts.Var", "org.wartremover.warts.Null"))
  def isSameTree(p: TreeNode, q: TreeNode): Boolean =
    (p, q) match {
      case (null, null) => true
      case (null, _)    => false
      case (_, null)    => false
      case _            => p.value == q.value && isSameTree(p.left, q.left) && isSameTree(p.right, q.right)
    }
}
