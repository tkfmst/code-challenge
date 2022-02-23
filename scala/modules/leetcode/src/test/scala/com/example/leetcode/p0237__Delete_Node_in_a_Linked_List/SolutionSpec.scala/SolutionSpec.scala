package com.example.leetcode.p0237

class SolutionSpec extends munit.FunSuite {
  import Solution._

  @SuppressWarnings(Array("org.wartremover.warts.Var", "org.wartremover.warts.Null"))
  private[this] def node2list(node: ListNode): List[Int] =
    if (node.next == null) List(node.x)
    else node.x +: node2list(node.next)

  test("""Input: head = [4,5,1,9], node=5 | Output: [4,1,9]""") {
    val node0 = new ListNode(4)
    val node1 = new ListNode(5)
    val node2 = new ListNode(1)
    val node3 = new ListNode(9)

    node0.next = node1
    node1.next = node2
    node2.next = node3

    deleteNode(node1)

    assertEquals(node2list(node0), List(4, 1, 9))
  }

  test("""Input: head = [4,5,1,9], node=1 | Output: [4,5,9]""") {
    val node0 = new ListNode(4)
    val node1 = new ListNode(5)
    val node2 = new ListNode(1)
    val node3 = new ListNode(9)

    node0.next = node1
    node1.next = node2
    node2.next = node3

    deleteNode(node2)

    assertEquals(node2list(node0), List(4, 5, 9))
  }
}
