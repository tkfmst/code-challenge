package com.example.leetcode.p100

class SolutionSpec extends munit.FunSuite {
  import Solution._

  test("""Input:  p = [1,2,3], q = [1,2,3] | Output: true""") {
    val left1  = new TreeNode(2, null, null)
    val right1 = new TreeNode(3, null, null)
    val p      = new TreeNode(1, left1, right1)

    val left2  = new TreeNode(2, null, null)
    val right2 = new TreeNode(3, null, null)
    val q      = new TreeNode(1, left2, right2)

    assertEquals(isSameTree(p, q), true)
  }

  test("""Input:  p = [1,2], q = [1,null,3] | Output: false""") {
    val left1 = new TreeNode(2, null, null)
    val p     = new TreeNode(1, left1, null)

    val right2 = new TreeNode(2, null, null)
    val q      = new TreeNode(1, null, right2)

    assertEquals(isSameTree(p, q), false)
  }

  test("""Input:  p = [1,2, 1], q = [1,1,2] | Output: false""") {
    val left1  = new TreeNode(2, null, null)
    val right1 = new TreeNode(1, null, null)
    val p      = new TreeNode(1, left1, right1)

    val left2  = new TreeNode(1, null, null)
    val right2 = new TreeNode(2, null, null)
    val q      = new TreeNode(1, left2, right2)

    assertEquals(isSameTree(p, q), false)
  }
}
