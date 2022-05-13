package com.example.leetcode.p0117


// format: off
/**
 * 117. Populating Next Right Pointers in Each Node II
 * Medium
 * 
 * Given a binary tree
 * 
 * struct Node {
 *   int val;
 *   Node *left;
 *   Node *right;
 *   Node *next;
 * }
 * 
 * Populate each next pointer to point to its next right node. If there is no next right node, the next pointer should be set to NULL.
 * 
 * Initially, all next pointers are set to NULL.
 * 
 * 
 * Example 1:
 * ![117_sample.png](./img/117_sample.png)
 * 
 * Input: root = [1,2,3,4,5,null,7]
 * Output: [1,#,2,3,#,4,5,7,#]
 * Explanation: Given the above binary tree (Figure A), your function should populate each next pointer to point to its next right node, just like in Figure B. The serialized output is in level order as connected by the next pointers, with '#' signifying the end of each level.
 * 
 * Example 2:
 * 
 * Input: root = []
 * Output: []
 * 
 * 
 * Constraints:
 * 
 * * The number of nodes in the tree is in the range [0, 6000].
 * * -100 <= Node.val <= 100
 * 
 * 
 * Follow-up:
 * - You may only use constant extra space.
 * - The recursive approach is fine. You may assume implicit stack space does not count as extra space for this problem.
 */
// format: on

object Solution {
  def connect(root: Node): Node = {
    def helper(nodeOption: Option[Node]): Unit = {
      connect(nodeOption, None, None) match {
        case Some(firstChild) => helper(Some(firstChild))
        case None             => ()
      }
    }

    helper(Option(root))
    root
  }

  @scala.annotation.tailrec
  private def connect(
      nodeOption: Option[Node],
      childOption: Option[Node],
      firstChildOption: Option[Node]
  ): Option[Node] = {
    nodeOption match {
      case Some(node) =>
        (Option(node.left), Option(node.right), childOption) match {
          case (Some(left), None, Some(child)) =>
            child.next = left
            connect(Option(node.next), Some(left), firstChildOption)

          case (Some(left), None, None) =>
            connect(Option(node.next), Some(left), Some(left))

          case (None, Some(right), Some(child)) =>
            child.next = right
            connect(Option(node.next), Some(right), firstChildOption)

          case (None, Some(right), None) =>
            connect(Option(node.next), Some(right), Some(right))

          case (Some(left), Some(right), Some(child)) =>
            child.next = left
            left.next = right
            connect(Option(node.next), Some(right), firstChildOption)

          case (Some(left), Some(right), None) =>
            left.next = right
            connect(Option(node.next), Some(right), Some(left))

          case _ =>
            connect(Option(node.next), childOption, firstChildOption)

        }

      case _ =>
        firstChildOption
    }
  }
}
