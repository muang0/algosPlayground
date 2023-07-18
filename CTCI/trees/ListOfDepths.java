package trees;

import linkedlist.LinkedList;

import java.util.ArrayList;
import java.util.Queue;

import static trees.MinimalTree.createMinimalTree;
import static trees.Node.breadthFirstSearch;

public class ListOfDepths {
    // Problem: Given a binary tree, design an algorithm which creates a linked list of all the nodes at each depth
    // Solution: Iterate through a list of nodes at the current layer, append their values to result linkedList & create nextNodeLayer list from their children--
    //  which will be iterated through next via recursion
    //  corresponding depth LinkedList depending on the counter value
    // Time Complexity: O(n) since we are only looping through the tree once
    // Space Complexity: O(n) since the linked lists size is linearly related to the input tree

    public static void main(String[] args){
        // first, create the tree from an input array using a previous problem
        int[] inputArray = {1,2,3,4,5,6,7,8,9};
        Node n = createMinimalTree(inputArray, 0, inputArray.length - 1);
        ArrayList<LinkedList> linkedListArray = createLinkedListsFromTree(n);
        // print results for visual verification
        System.out.println("BFS results of input");
        breadthFirstSearch(n);
        System.out.println("Print output results");
        for(int depth = 0; depth < linkedListArray.size(); depth++){
            LinkedList linkedList = linkedListArray.get(depth);
            LinkedList.Node node = linkedList.head;
            System.out.println("Depth layer: "  + depth);
            while(node != null){
                System.out.println(node.data);
                node = node.next;
            }
        }
    }

    public static ArrayList<LinkedList> createLinkedListsFromTree(Node n) {
        ArrayList<LinkedList> linkedListArray = new ArrayList<>();
        LinkedList linkedList;
        Queue<Node> currentQueue = new java.util.LinkedList<>();
        Queue<Node> nextQueue;
        currentQueue.add(n);
        LinkedList.Node previousNode = null, currentLinkedListNode = null;
        while (currentQueue.size() > 0) {
            linkedList = new LinkedList();
            nextQueue = new java.util.LinkedList<>();
            linkedListArray.add(linkedList);
            boolean isCurrentNodeHead = true;
            for(Node currentNode: currentQueue) {
                currentLinkedListNode = linkedList.new Node(currentNode.value);
                if(isCurrentNodeHead){ linkedList.head = currentLinkedListNode; isCurrentNodeHead = false; }
                else { previousNode.next = currentLinkedListNode; }
                if (currentNode.left != null) { nextQueue.add(currentNode.left); }
                if (currentNode.right != null) { nextQueue.add(currentNode.right); }
                previousNode = currentLinkedListNode;
            }
            currentQueue = nextQueue;
        }
        return linkedListArray;
    }

}
