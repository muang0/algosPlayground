package trees;

import static trees.Node.printTreeInOrderDFS;

public class MinimalTree {
    // Problem: given a sorted (ascending) array with unique integer elements, write algorithm to create BST with minimal height
    // Solution: create a function that sets middle of input array to be node, then sets left and right partitions to be left and
    //  right trees & recurse
    // Time Complexity: O(n) since we are only looping through the list once
    // Space Complexity: O(n) since the tree size is linearly related to the input list

    public static void main(String args[]){
        int[] inputArray = {1,2,3,4,5,6,7,8,9};
        Node n = createMinimalTree(inputArray, 0, inputArray.length - 1);
        printTreeInOrderDFS(n);
    }

    public static Node createMinimalTree(int[] inputArray, int lowerIndex, int upperIndex) {
        if(upperIndex < lowerIndex) {
            return null;
        }
        int middleIndex = lowerIndex + (upperIndex - lowerIndex) / 2;
        Node n = new Node(inputArray[middleIndex]);
        n.left = createMinimalTree(inputArray, lowerIndex, middleIndex - 1);
        n.right = createMinimalTree(inputArray, middleIndex + 1, upperIndex);
        return n;
    }

}
