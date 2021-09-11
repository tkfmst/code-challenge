package com.example.leetcode.p21

/**
 * https://leetcode.com/problems/merge-two-sorted-lists/
 *
 * Merge two sorted linked lists and return it as a sorted list. The list should be made by splicing together the nodes
 * of the first two lists.
 */

/**
 * Usually write like this
 */
// import scala.annotation.tailrec
//
// sealed trait ListNode
// object ListNode {
//   final case class One(x: Int, next: ListNode) extends ListNode
//   final case class Empty()                     extends ListNode
//
//   val empty: ListNode = Empty()
//
//   def create(sq: Seq[Int]): ListNode =
//     sq.reverse.foldLeft(empty)((nxt, i) => One(i, nxt))
// }
//
// object Solution {
//   def mergeTwoLists(l1: ListNode, l2: ListNode): ListNode = {
//
//     @tailrec
//     def extract(ln: ListNode, acc: Seq[Int]): Seq[Int] =
//       ln match {
//         case _: ListNode.Empty => acc
//         case o: ListNode.One   => extract(o.next, acc :+ o.x)
//       }
//
//     ListNode.create((extract(l1, Seq()) ++ extract(l2, Seq())).sorted)
//   }
// }

/**
 * Provided Code
 */
@SuppressWarnings(Array("org.wartremover.warts.Var", "org.wartremover.warts.Null"))
class ListNode(_x: Int = 0, _next: ListNode = null) {
  var next: ListNode = _next
  var x: Int         = _x

  // for test
  override def equals(that: Any): Boolean = that match {
    case node: ListNode => this.x == node.x && this.next == node.next
    case _              => false
  }
}

@SuppressWarnings(Array("org.wartremover.warts.Null", "org.wartremover.warts.TraversableOps"))
object ListNodeBuilder {
  private[this] def create(i: Int, nxt: ListNode) = new ListNode(i, nxt)

  def build(sq: Seq[Int]): ListNode =
    if (sq.isEmpty) null
    else {
      val rev = sq.reverse
      rev.tail.foldLeft(create(rev.head, null))((next, i) => create(i, next))
    }
}

@SuppressWarnings(Array("org.wartremover.warts.Null", "org.wartremover.warts.TraversableOps"))
object Solution {
  def mergeTwoLists(l1: ListNode, l2: ListNode): ListNode = {

    @scala.annotation.tailrec
    def extract(node: ListNode, acc: Seq[Int] = Seq.empty): Seq[Int] =
      if (node == null) acc
      else if (node.next == null) acc :+ node.x
      else extract(node.next, acc :+ node.x)

    ListNodeBuilder.build((extract(l1) ++ extract(l2)).sorted)
  }
}
