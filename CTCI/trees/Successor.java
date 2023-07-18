package trees;

public class Successor {
    // Problem: Write an algorithm to find the 'next' node of a given node in a BST
    // Solution: return the first non-null of the following cases:
    //  1. furthest left node of the right sub-node of n
    //  2. the right sub-node of n
    //  3. the parent node (for this problem we are given link to parent node)
    // Time Complexity: O(n) since we are only performing one loop
    // Space Complexity: O(n) We are only storing the input tree

    public static void main(String[] args){
//        dummy BST
//                     0
//                        \
//                         3
//                        /  \
//                        2    4
//                         \
//                          8
        Node n = new Node(0);
        Node parent = new Node(66);
        n.right = new Node(3);
        n.right.left = new Node(2);
        n.right.right = new Node(8);
        n.right.right = new Node(4);
        System.out.println("Expected value 2: " + returnSuccessor(n, parent).value);
        System.out.println("Expected value 4: " + returnSuccessor(n.right, parent).value);
        System.out.println("Expected value 66: " + returnSuccessor(n.right.right, parent).value);

    }

    public static Node returnSuccessor(Node n, Node parent){
        // cases 1 & 2
        if(n.right != null){
            n = n.right;
            while(n.left != null){ n = n.left; }
            return n;
        }
        // case 3
        return parent;
    }
}
