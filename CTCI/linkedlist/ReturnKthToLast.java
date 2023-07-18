package linkedlist;

public class ReturnKthToLast {
    // Problem: Return the data at node k from the end of a singly linked list
    // Solution: Iterate through linked list with two pointers, one is k elements behind the other
    //  when the first one hits null, we know the second is the kth to last element
    // Time Complexity: O(n) since we are only looping through the list once
    // Space Complexity: O(n) since we are only storing the input linkedList

    public static void main(String[] args){
        LinkedListExample1 linkedList = new LinkedListExample1();
        LinkedList.Node n1 = linkedList.getHead();
        LinkedList.Node n2 = linkedList.getHead();
        int k = 8, counter = 0;
        while (n1.next != null) {
            if (counter >= k) {
                n2 = n2.next;
            }
            n1 = n1.next;
            counter++;
        }
        System.out.println("Data at node " + k + "'th to end is: " + n2.data);
    }
}
