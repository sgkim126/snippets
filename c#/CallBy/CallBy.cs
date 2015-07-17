using System.Collections.Generic;
class CallBy
{
    static void Main(string[] args)
    {
        List<int> list ;

        list = new List<int>();
        list.Add(0);
        CallByReference(ref list);
        Print("Call by Reference", list);

        list = new List<int>();
        list.Add(0);
        CallByReference2(ref list);
        Print("Call by Reference2", list);

        list = new List<int>();
        list.Add(0);
        CallBySharing(list);
        Print("Call by Sharing", list);

        list = new List<int>();
        list.Add(0);
        CallBySharing2(list);
        Print("Call by Sharing2", list);
    }
    static void CallByReference(ref List<int> list)
    {
        list.Add(1);
    }
    static void CallByReference2(ref List<int> list)
    {
        list = new List<int>();
        list.Add(1);
    }
    static void CallBySharing(List<int> list)
    {
        list.Add(1);
    }
    static void CallBySharing2(List<int> list)
    {
        list = new List<int>();
        list.Add(1);
    }
    static void Print(string name, List<int> list)
    {
        System.Console.WriteLine(name);
        foreach (int element in list)
        {
            System.Console.WriteLine(element);
        }
    }
}