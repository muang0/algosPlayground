package linkedlist;
import java.util.HashMap;
import static linkedlist.LinkedList.printLinkedList;


public class RemoveDups {
    // Problem: Remove duplicates from a linkedList
    // Solution: Loop through the list and use a hashmap to store data values encountered thus far
    //  if a value has been encountered, remove the node from the list
    // Time Complexity: O(n) since we are only looping through the list once
    // Space Complexity: O(n) since we are only storing the input linkedList

    public static void main(String[] args) {
        LinkedListExample1 linkedList = new LinkedListExample1();
        LinkedList.Node node = linkedList.getHead();
        LinkedList.Node laggingNode = null;
        HashMap<Integer, Integer> map = new HashMap<Integer, Integer>();
	    while (node != null) {
	        System.out.println(node.data);
	        if (map.containsKey(node.data)) {
                laggingNode.next = node.next;
            }
	        else {
                map.put(node.data, node.data);
                laggingNode = node;
            }
            node = node.next;
        }

	    // print linked list for confirmation
        // revisit
//        printLinkedList(linkedList);

    }

}
