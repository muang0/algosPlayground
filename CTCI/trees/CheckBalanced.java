package trees;

import linkedlist.LinkedList;

import java.util.ArrayList;
import java.util.Queue;

import static trees.MinimalTree.createMinimalTree;
import static trees.Node.breadthFirstSearch;

public class CheckBalanced {
    // Problem: Implement a function to check if a binary tree is balanced.  For this question a balanced tree is a tree
    //  where the heights of the twi subtrees of any node never differ by more than one
    // Solution: calculate the max height for each node in the tree using DFS, if any differs by more than 1 we know we don't have a complete tree
    // Time Complexity: O(n) because we are doing a breadth-first search
    // Space Complexity: O(n) because we are only storing the input tree


    public static void main(String[] args){
        int[] inputArray = {1,2,3,4,5,6,7,8,9};
        Node n = createMinimalTree(inputArray, 0, inputArray.length - 1);
        // test with balanced tree
//        System.out.println(isTreeBalanced(n));
        // test with rudimentary unbalanced tree
//                    0
//                  /   \
//                  1     2
//                 / \     \
//                3   4     3
//               /           \
//              5             4
//             /
//            6
        n = new Node(0);
        n.left = new Node(1);
        n.left.left = new Node(3);
        n.left.left.left = new Node(5);
        n.left.left.left.left = new Node(6);
        n.left.right = new Node(4);
        n.right = new Node(2);
        n.right.right = new Node(3);
        n.right.right.right = new Node(4);
        System.out.println(isTreeBalanced(n));
    }

    public static boolean isTreeBalanced(Node n) {
        // recurse through the tree, if lengths of left & right node differ by more than 1 return false
        if(n != null) { if(Math.abs(getHeight(n.left) - getHeight(n.right)) > 1){ return false;} }
        if(n.left != null){ return isTreeBalanced(n.left); }
        if(n.right != null){ return isTreeBalanced(n.right); }
        return true;
    }

    private static int getHeight(Node n){
        if(n == null){ return -1; }
        return Math.max(getHeight(n.left), getHeight(n.right)) + 1;
    }

}
