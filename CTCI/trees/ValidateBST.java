package trees;


public class ValidateBST {
    // Problem: Implement a function to check if a binary tree is a binary search tree
    // Solution: DFS search, if current number is smaller than previous then return false
    // Time Complexity: O(n) since we are only looping through the tree once
    // Space Complexity: O(n) We are only storing the input tree

    public static void main(String[] args){
//        dummy BST
//                     0
//                  /     \
//                 -2      3
//                 / \    /  \
//               -3  -1   2    4
        Node n = new Node(0);
        n.left = new Node(-2);
        n.left.left = new Node(-3);
        n.left.right = new Node(-1);
        n.right = new Node(3);
        n.right.left = new Node(2);
        n.right.right = new Node(4);
        System.out.println("Expected value true: " + (isNodeBST(n, Integer.MIN_VALUE) != null ? true : false));
//        dummy non-valid BST-- swap -2 -> -7
        n.left.value = -7;
        // return value is the value at head if tree is BST, else it is null
        System.out.println("Expected value false: " + (isNodeBST(n, Integer.MIN_VALUE) != null ? true : false));
    }

    public static Integer isNodeBST(Node n, Integer previousVal){
        // In-order traversal of the tree where we validate current value is greater or equal to previous value
        if(n != null){
            previousVal = isNodeBST(n.left, previousVal);
            if(previousVal == null){ return null;}
            else if(n.value < previousVal){ return null; }
            previousVal = isNodeBST(n.right, previousVal == null? null: n.value);
        }
        return previousVal;
    }

}
