package trees;

import java.util.LinkedList;
import java.util.Queue;

public class Node {
    public int value;
    public Node left;
    public Node right;

    public Node(int value){
        this.value = value;
    }

    public static void breadthFirstSearch(Node n){
        Queue<Node> bfsQueue = new LinkedList<>();
        bfsQueue.add(n);
        while(!bfsQueue.isEmpty()){
            Node currentNode = bfsQueue.remove();
            System.out.println(currentNode.value);
            if(currentNode.left != null) {
                bfsQueue.add(currentNode.left);
            }
            if(currentNode.right != null) {
                bfsQueue.add(currentNode.right);
            }
        }
    }

    public static void printTreeInOrderDFS(Node n){
        if(n != null){
            printTreeInOrderDFS(n.left);
            System.out.println(n.value);
            printTreeInOrderDFS(n.right);
        }
    }

    public static void printTreePreOrderDFS(Node n){
        if(n != null){
            System.out.println(n.value);
            printTreeInOrderDFS(n.left);
            printTreeInOrderDFS(n.right);
        }
    }

    public static void printTreePostOrderDFS(Node n){
        if(n != null){
            printTreeInOrderDFS(n.left);
            printTreeInOrderDFS(n.right);
            System.out.println(n.value);
        }
    }

}
