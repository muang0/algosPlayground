package linkedlist;

public class LinkedListExample2 {
    // This class represents an example instantiated linkedList for testing/demonstration purposes

    LinkedList linkedList1 = new LinkedList();
    LinkedList.Node n1 = linkedList1.new Node(3);
    LinkedList.Node n2 = linkedList1.new Node(8);
    LinkedList.Node n3 = linkedList1.new Node(3);
    LinkedList.Node n4 = linkedList1.new Node(9);
    LinkedList.Node n5 = linkedList1.new Node(9);
    LinkedList.Node n6 = linkedList1.new Node(3);
    LinkedList.Node n7 = linkedList1.new Node(2);

    public LinkedListExample2() {
        linkedList1.head = n1;
        n1.next = n2;
        n2.next = n3;
        n3.next = n4;
        n4.next = n5;
        n5.next = n6;
        n6.next = n7;
    }

    public LinkedList.Node getHead() {
        return linkedList1.head;
    }

    public void setHead(LinkedList.Node node) {
        node.next = getHead();
        linkedList1.head = node;
    }
}
