package com.example.leetcode.p0160

class SolutionSpec extends munit.FunSuite {
  import Solution._

  test("""Input: intersectVal = 8, listA = [4,1,8,4,5], listB = [5,6,1,8,4,5], skipA = 2, skipB = 3 | Output: 8""") {

    val aNode0 = new ListNode(4)
    val aNode1 = new ListNode(1)

    val bNode0 = new ListNode(5)
    val bNode1 = new ListNode(6)
    val bNode2 = new ListNode(1)

    val intersectNode0 = new ListNode(8)
    val intersectNode1 = new ListNode(4)
    val intersectNode2 = new ListNode(5)

    intersectNode0.next = intersectNode1
    intersectNode1.next = intersectNode2

    aNode0.next = aNode1
    aNode1.next = intersectNode0

    bNode0.next = bNode1
    bNode1.next = bNode2
    bNode2.next = intersectNode0

    assertEquals(getIntersectionNode(aNode0, bNode0), intersectNode0)
  }

  test("""Input: intersectVal = 2, listA = [1,9,1,2,4], listB = [3,2,4], skipA = 3, skipB = 1 | Output: 2""") {

    val aNode0 = new ListNode(1)
    val aNode1 = new ListNode(9)
    val aNode2 = new ListNode(1)

    val bNode0 = new ListNode(3)

    val intersectNode0 = new ListNode(2)
    val intersectNode1 = new ListNode(4)

    intersectNode0.next = intersectNode1

    aNode0.next = aNode1
    aNode1.next = aNode2
    aNode2.next = intersectNode0

    bNode0.next = intersectNode0

    assertEquals(getIntersectionNode(aNode0, bNode0), intersectNode0)
  }

  test("""Input: intersectVal = 0, listA = [2,6,4], listB = [1,5], skipA = 3, skipB = 2 | Output: null""") {

    val aNode0 = new ListNode(2)
    val aNode1 = new ListNode(6)
    val aNode2 = new ListNode(4)

    val bNode0 = new ListNode(1)
    val bNode1 = new ListNode(5)

    aNode0.next = aNode1
    aNode1.next = aNode2

    bNode0.next = bNode1

    assertEquals(getIntersectionNode(aNode0, bNode0), null)
  }
}
