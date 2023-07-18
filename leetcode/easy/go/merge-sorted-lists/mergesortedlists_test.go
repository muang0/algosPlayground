package mergelists

import (
	"testing"
)

func convertlltoarray(ll *ListNode) []int {
	var out []int
	for ll != nil {
		out = append(out, ll.Val)
		ll = ll.Next
	}
	return out
}

// next steps: create function to convert array to linkedlist

func TestMergeTwoLists(t *testing.T) {
	n3 := ListNode{Val: 4, Next: nil}
	n2 := ListNode{Val: 2, Next: &n3}
	n1 := ListNode{Val: 1, Next: &n2}
	n6 := ListNode{Val: 4, Next: nil}
	n5 := ListNode{Val: 3, Next: &n6}
	n4 := ListNode{Val: 1, Next: &n5}
	result := mergeTwoLists(&n1, &n4)
	resultarr := convertlltoarray(result)
	expected := []int{1, 1, 2, 3, 4, 4}
	for i := range expected {
		if i >= len(resultarr) {
			t.Errorf("expected %+v got %+v", expected, resultarr)
			break
		}
		if resultarr[i] != expected[i] {
			t.Errorf("expected %+v got %+v", expected, resultarr)
		}
	}
}
