package linkedlist;

import java.util.ArrayList;
import java.util.List;
import static linkedlist.LinkedList.printLinkedList;

public class DeleteMiddleNode {
    // Problem: Implement an algorithm to delete a node in the middle of a linked list
    //  we are only given access to the middle of the list, not the head
    // Solution: Starting from the middle node, loop to the end of the node and set the current nodes data equal to the next nodes data
    // Time Complexity: O(n) since we are doing one loop
    // Space Complexity: O(n) since we are only storing the input linkedList

    public static void main(String[] args) {
        LinkedListExample1 linkedList = new LinkedListExample1();
        List<LinkedList.Node> middleElement = getMiddleOfLinkedList(linkedList);
        // get the middle node (for this example we will just take first of the middle nodes if there are multiple)
        LinkedList.Node node = middleElement.get(0);
        while (node.next != null) {
            node.data = node.next.data;
            node = node.next;
        }
        // revisit
//        printLinkedList(linkedList);
    }

    private static List<LinkedList.Node> getMiddleOfLinkedList(LinkedListExample1 linkedList) {
        // Problem: Get the middle of a linked list with only one iteration
        // Solution: Have two iterators, second goes through the list at half the speed of the first
        //  when the first hits null we know the second is at the middle (or one of the middle nodes depending on if
        //  there is an even or odd total number of nodes)
        int counter = 1;
        LinkedList.Node n1 = linkedList.getHead();
        LinkedList.Node n2 = linkedList.getHead();
        while (n1.next != null) {
            n1 = n1.next;
            // every other iteration, increment n2 node
            if (counter % 2 == 0) {
                n2 = n2.next;
            }
            counter++;
        }
        // if counter is even, then we need to return the two middle nodes (n2 and n2.next)
        List<LinkedList.Node> returnArray = new ArrayList<LinkedList.Node>();
        returnArray.add(n2);
        if (counter % 2 == 0) {
            returnArray.add(n2.next);
        }
        return returnArray;
    }
}
