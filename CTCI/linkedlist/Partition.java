package linkedlist;

import static linkedlist.LinkedList.printLinkedList;

public class Partition {
    // Problem: Write code to partition a linked list around a value x, such that all nodes less than x come before all nodes
    //  greater than or equal to 'x'
    //  note that they don't have to be in any numeric order, just partitioned based on value 'x'
    // Solution: Loop through once, if any nodes with data less than x are encountered move them at front of linkedList
    // Time Complexity: O(n) since we are only looping through the list once
    // Space Complexity: O(n) since we are only storing the input linkedList

    public static void main(String[] args) {
        LinkedListExample1 linkedList = new LinkedListExample1();
        LinkedList.Node n1 = linkedList.getHead();
        LinkedList.Node lagNode = linkedList.getHead();
        LinkedList.Node nextNode = null;
        int x = 10, counter = 0;
        while (n1 != null) {
            nextNode = n1.next;
            // ignore the head node
            if (counter > 0) {
                if (n1.data < x) {
                    // move n1 to front of linkedList
                    n1.next = linkedList.getHead();
                    linkedList.setHead(n1);
                    // remove n1's previous location from linkedList
                    lagNode.next = nextNode;
                }
                else {
                    lagNode = lagNode.next;
                }
            }
            n1 = nextNode;
            counter++;
        }
        // revisit
//        printLinkedList(linkedList);
    }
}
