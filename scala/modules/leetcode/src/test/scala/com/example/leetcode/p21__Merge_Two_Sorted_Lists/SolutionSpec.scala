package com.example.leetcode.p21

class SolutionSpec extends munit.FunSuite {
  import Solution._

  /**
   * Usually write like this
   */
  // test("Input: l1 = [], l2 = [] | Output: []") {
  //   assertEquals(mergeTwoLists(ListNode.create(Seq()), ListNode.create(Seq())), ListNode.create(Seq()))
  // }
  //
  // test("Input: l1 = [], l2 = [0] | Output: [0]") {
  //   assertEquals(mergeTwoLists(ListNode.create(Seq()), ListNode.create(Seq(0))), ListNode.create(Seq(0)))
  // }
  //
  // test("Input: l1 = [1, 2, 4], l2 = [1, 3, 4] | Output: [1, 2, 2, 3, 4, 4]") {
  //   assertEquals(
  //     mergeTwoLists(ListNode.create(Seq(1, 2, 4)), ListNode.create(Seq(1, 3, 4))),
  //     ListNode.create(Seq(1, 1, 2, 3, 4, 4))
  //   )
  // }

  test("Input: l1 = [], l2 = [] | Output: []") {
    assertEquals(
      mergeTwoLists(ListNodeBuilder.build(Seq()), ListNodeBuilder.build(Seq())),
      ListNodeBuilder.build(Seq())
    )
  }

  test("Input: l1 = [], l2 = [0] | Output: [0]") {
    assertEquals(
      mergeTwoLists(ListNodeBuilder.build(Seq()), ListNodeBuilder.build(Seq(0))),
      ListNodeBuilder.build(Seq(0))
    )
  }

  test("Input: l1 = [1, 2, 4], l2 = [1, 3, 4] | Output: [1, 2, 2, 3, 4, 4]") {
    val obtained = mergeTwoLists(ListNodeBuilder.build(Seq(1, 2, 4)), ListNodeBuilder.build(Seq(1, 3, 4)))
    val expected = ListNodeBuilder.build(Seq(1, 1, 2, 3, 4, 4))

    assert(obtained == expected)
  }
}
