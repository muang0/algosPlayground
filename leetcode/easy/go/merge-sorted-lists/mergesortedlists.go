package mergelists

// https://leetcode.com/problems/merge-two-sorted-lists/

// ListNode represents node in linked list
type ListNode struct {
	Val  int
	Next *ListNode
}

// Runtime: 0 ms, faster than 100.00% of Go online submissions for Merge Two Sorted Lists.
// Memory Usage: 2.6 MB, less than 7.14% of Go online submissions for Merge Two Sorted Lists.
// next steps: figure how to solve without having extra node heading the list

func mergeTwoLists(l1 *ListNode, l2 *ListNode) *ListNode {
	node := ListNode{}
	n := &node

	for l1 != nil && l2 != nil {
		node2 := new(ListNode)
		n.Next = node2
		n = node2

		if l1.Val < l2.Val {
			n.Val = l1.Val
			l1 = l1.Next
		} else {
			n.Val = l2.Val
			l2 = l2.Next
		}
	}

	if l1 == nil {
		n.Next = l2
	} else if l2 == nil {
		n.Next = l1
	}

	return node.Next
}
