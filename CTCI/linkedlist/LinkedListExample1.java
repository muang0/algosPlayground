package linkedlist;

public class LinkedListExample1 {
    // This class represents an example instantiated linkedList for testing/demonstration purposes

    int[] linkedListArray = {3,11,4,4,4,4,8,8,8,23,23,23,4,5,7,1,4};
    LinkedList linkedList1 = new LinkedList();
    LinkedList.Node n1 = linkedList1.new Node(3);
    LinkedList.Node n2 = linkedList1.new Node(1);
    LinkedList.Node n3 = linkedList1.new Node(4);
    LinkedList.Node n4 = linkedList1.new Node(5);
    LinkedList.Node n5 = linkedList1.new Node(4);
    LinkedList.Node n6 = linkedList1.new Node(8);
    LinkedList.Node n7 = linkedList1.new Node(3);
    LinkedList.Node n8 = linkedList1.new Node(8);
    LinkedList.Node n9 = linkedList1.new Node(8);
    LinkedList.Node n10 = linkedList1.new Node(18);

    public LinkedListExample1() {
        linkedList1.head = n1;
        n1.next = n2;
        n2.next = n3;
        n3.next = n4;
        n4.next = n5;
        n5.next = n6;
        n6.next = n7;
        n7.next = n8;
        n8.next = n9;
        n9.next = n10;
    }

    public LinkedList.Node getHead() {
        return linkedList1.head;
    }

    public void setHead(LinkedList.Node node) {
        node.next = getHead();
        linkedList1.head = node;
    }
}
