#include <iostream>
#include <string>
#include <vector>
using std::vector;
using std::cout;
using std::endl;
using std::string;
void print(string name, vector<int> list) {
  cout<<name<<endl;
  for(int ii = 0;ii < list.size();ii++) {
    cout<<list.at(ii)<<endl;
  }
}
void call_by_value(vector<int> list){
  list.push_back(1);
}
void call_by_value2(vector<int> list){
  list = vector<int>();
  list.push_back(1);
}
void call_by_reference(vector<int>& list){
  list.push_back(1);
}
void call_by_reference2(vector<int>& list){
  list = vector<int>();
  list.push_back(1);
}
void call_by_sharing(vector<int>* list){
  list->push_back(1);
}
void call_by_sharing2(vector<int>* list){
  list = new vector<int>();
  list->push_back(1);
}

int main(void) {
  vector<int> list;

  list = vector<int>();
  list.push_back(0);
  call_by_value(list);
  print("call by value", list);

  list = vector<int>();
  list.push_back(0);
  call_by_value2(list);
  print("call by value2", list);

  list = vector<int>();
  list.push_back(0);
  call_by_reference(list);
  print("call by reference", list);

  list = vector<int>();
  list.push_back(0);
  call_by_reference2(list);
  print("call by reference2", list);

  list = vector<int>();
  list.push_back(0);
  call_by_sharing(&list);
  print("call by sharing", list);

  list = vector<int>();
  list.push_back(0);
  call_by_sharing2(&list);
  print("call by sharing2", list);
}
