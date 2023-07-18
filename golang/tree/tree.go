package tree

import "fmt"

type node struct {
	val   int
	lnode *node
	rnode *node
}

type tree struct {
	head *node
}

func (t tree) printtreeinorder() {
	PrintInOrder(t.head)
}

// PrintInOrder starting from node n
func PrintInOrder(n *node) {
	if n.lnode != nil {
		PrintInOrder(n.lnode)
	}
	fmt.Println(n.val)
	if n.rnode != nil {
		PrintInOrder(n.rnode)
	}
}
