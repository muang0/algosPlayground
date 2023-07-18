package linkedlist

import "fmt"

// implement a singly linkedList in go

type node struct {
	val  int
	next *node
}

type linkedlist struct {
	head *node
}

func (ll linkedlist) print() {
	node := ll.head
	var out []int
	for {
		out = append(out, node.val)
		if node.next == nil {
			break
		}
		node = node.next
	}
	fmt.Println(out)
}
