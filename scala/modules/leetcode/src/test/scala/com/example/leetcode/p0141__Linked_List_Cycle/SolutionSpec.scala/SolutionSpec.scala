package com.example.leetcode.p0141

class SolutionSpec extends munit.FunSuite {
  import Solution._

  test("""Input: head = [3,2,0,-4], pos=1 | Output: true""") {
    val node0 = new ListNode(3)
    val node1 = new ListNode(2)
    val node2 = new ListNode(0)
    val node3 = new ListNode(-4)

    node0.next = node1
    node1.next = node2
    node2.next = node3
    node3.next = node1

    assertEquals(hasCycle(node0), true)
  }

  test("""Input:head = [1,2], pos=0 | Output: true""") {
    val node0 = new ListNode(1)
    val node1 = new ListNode(2)

    node0.next = node1
    node1.next = node0

    assertEquals(hasCycle(node0), true)
  }

  test("""Input: head = [1], pos=-1 | Output: false""") {
    val node0 = new ListNode(1)

    assertEquals(hasCycle(node0), false)
  }

  test("""Input: head = [], pos=-1 | Output: false""") {
    val node0: ListNode = null

    assertEquals(hasCycle(node0), false)
  }
}
