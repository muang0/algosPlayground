package linkedlist;

import java.lang.reflect.Array;
import java.util.ArrayList;

import static linkedlist.LinkedList.convertArrayToLinkedList;
import static linkedlist.LinkedList.printLinkedList;

public class SumLists {

    public static void main(String[] args) {
        LinkedList linkedListOne = convertArrayToLinkedList(new int[]{3, 4, 2});
        LinkedList linkedListTwo = convertArrayToLinkedList(new int[]{8, 2, 9, 8, 3, 4});
//        printLinkedList(sumTheListsInReverseOrder(linkedListOne, linkedListTwo));
        printLinkedList(sumTheListsInStandardOrder(linkedListOne, linkedListTwo));
    }

    private static LinkedList sumTheListsInReverseOrder(LinkedList linkedListOne, LinkedList linkedListTwo) {
        //  Problem: Two numbers represented by linked list, where each node contains a single digit.  The digits are
        //  stored in reverse order such that the 1's digit is at the head of the list
        //  Solution: Iterate through linked list with two pointers, one is k elements behind the other
        //  when the first one hits null, we know the second is the kth to last element
        //  Time Complexity: O(n) since we are only looping through the list once
        //  Space Complexity: O(n) since we are only storing the input linkedList
        LinkedList.Node LL1Node = linkedListOne.head;
        LinkedList.Node LL2Node = linkedListTwo.head;
        LinkedList sumLinkedList = new LinkedList();
        int sum, carry = 0;
        LinkedList.Node lagNode = null;
        while (true) {
            // calculate sum
            if (LL1Node == null) { sum = LL2Node.data + carry; }
            else if (LL2Node == null) { sum = LL1Node.data + carry; }
            else { sum = LL1Node.data + LL2Node.data + carry; }
            // calculate carry
            carry = (int) Math.floor(sum / 10);
            // assign sum to new node in return linked list
            LinkedList.Node node = sumLinkedList.new Node(sum);
            if (lagNode != null) { lagNode.next = node; }
            else { sumLinkedList.head = node; }
            lagNode = node;
            // increment nodes
            if (LL1Node != null) { LL1Node = LL1Node.next; }
            if (LL2Node != null) { LL2Node = LL2Node.next; }
            // break if at the end of both nodes
            if (LL1Node == null && LL2Node == null) { break; }
        }
        return sumLinkedList;
    }

    private static LinkedList sumTheListsInStandardOrder(LinkedList linkedListOne, LinkedList linkedListTwo) {
        //  Problem: Two numbers represented by linked list, where each node contains a single digit.  The digits are
        //  stored in standard order such that the 1's digit is at the end of the list
        //  Solution: Extract both lists into an ArrayList, determine the length difference between the two arrays,
        //  and loop through arrays starting at 1's place and calculate the sum
        //  Time Complexity: O(n)
        //  Space Complexity: O(n) since we are storing the input linkedLists and their array conversions
        // We could solve this using less space without the arrays, but would require finding the length of both linkedLists first
        // extract linked lists into arrays in reverse order
        LinkedList.Node LL1Node = linkedListOne.head;
        ArrayList<Integer> LL1Array = new ArrayList();
        while (true) {
            LL1Array.add(LL1Node.data);
            if (LL1Node.next == null) {
                break;
            }
            LL1Node = LL1Node.next;
        }
        System.out.println(LL1Array);
        LinkedList.Node LL2Node = linkedListTwo.head;
        ArrayList<Integer> LL2Array = new ArrayList();
        while (true) {
            LL2Array.add(LL2Node.data);
            if (LL2Node.next == null) {
                break;
            }
            LL2Node = LL2Node.next;
        }
        System.out.println(LL2Array);
        // determine offset (for matching arrays 'right to left')
        int offset = LL1Array.size() - LL2Array.size();
        int arrayIndexLL1 = LL1Array.size() - 1;
        ArrayList sumArray = new ArrayList();
        Integer sum, carry = 0;
        LinkedList.Node lagNode = null;
        LinkedList sumLinkedList = new LinkedList();
        while (true) {
            // loop through arrays right to left and add with carry
            if (arrayIndexLL1 < 0 && arrayIndexLL1 - offset < 0) { break; }
            else if (arrayIndexLL1 < 0) { sum = LL2Array.get(arrayIndexLL1 - offset) + carry; }
            else if (arrayIndexLL1 - offset < 0) { sum = LL1Array.get(arrayIndexLL1) + carry; }
            else { sum = LL2Array.get(arrayIndexLL1 - offset) + LL1Array.get(arrayIndexLL1) + carry; }
            carry = 0;
            if (sum > 9) { carry = 1; sum = sum % 10; }
            LinkedList.Node node = sumLinkedList.new Node(sum);
            if (lagNode != null) {
                node.next = lagNode;
            }
            lagNode = node;
            sumLinkedList.head = node;
            arrayIndexLL1--;
        }
        return sumLinkedList;
    }
}
