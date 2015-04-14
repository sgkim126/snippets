import java.util.*;


public class CallBySharing {
    public static void main(String[] args) {
        LinkedList<Integer> list = new LinkedList<Integer>();
        list.addLast(new Integer(1));
        callBySharing(list);
        ListIterator<?> iter = list.listIterator(0);
        while(iter.hasNext())
        {
            System.out.println(iter.next()); // 1, 2
        }
        callBySharing2(list);
        list.addLast(new Integer(3));
        iter = list.listIterator(0);
        while(iter.hasNext())
        {
            System.out.println(iter.next()); // 1, 2, 3
        }
    }
    public static void callBySharing(LinkedList<Integer> list) {
        list.addLast(new Integer(2));
    }
    public static void callBySharing2(LinkedList<Integer> list) {
        list = new LinkedList<Integer>();
        list.addLast(new Integer(4));
    }
}
