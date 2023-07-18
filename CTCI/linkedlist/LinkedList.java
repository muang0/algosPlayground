package linkedlist;


public class LinkedList {
    public Node head;

    public class Node {
        public int data;
        public Node next;

        public Node(int data) {
            this.data = data;
        }
    }

    public static LinkedList convertArrayToLinkedList(int[] arrayToBeConverted) {
        LinkedList returnLinkedList = new LinkedList();
        Node laggingNode = null;
        for (int i = 0; i < arrayToBeConverted.length; i++) {
            Node n1 = returnLinkedList.new Node(arrayToBeConverted[i]);
            if (laggingNode != null) {
                laggingNode.next = n1;
            }
            else { returnLinkedList.head = n1; }
            laggingNode = n1;
        }
        return returnLinkedList;
    }

    public static void printLinkedList(LinkedList linkedList) {
        LinkedList.Node node = linkedList.head;
        System.out.println("Printing linkedList!");
        while (node != null) {
            System.out.println(node.data);
            node = node.next;
        }
    }

}
