package tree

// example tree from geeksforgeeks
// https://www.geeksforgeeks.org/tree-traversals-inorder-preorder-and-postorder/
// example function utilized for testing https://blog.golang.org/examples

func ExamplePrintInOrder() {
	n3 := node{val: 3, lnode: nil, rnode: nil}
	n4 := node{val: 4, lnode: nil, rnode: nil}
	n5 := node{val: 5, lnode: nil, rnode: nil}
	n2 := node{val: 2, lnode: &n4, rnode: &n5}
	n1 := node{val: 1, lnode: &n2, rnode: &n3}
	PrintInOrder(&n1)
	// Output:
	// 4
	// 2
	// 5
	// 1
	// 3
}
