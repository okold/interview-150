#include <iostream>
using namespace std;


struct ListNode {
     int val;
     ListNode *next;
     ListNode(int x) : val(x), next(NULL) {}
};

bool hasCycle(ListNode *head) {

    ListNode *fast = head;
    ListNode *slow = head;

    while (slow != NULL && fast != NULL && fast->next != NULL) {
        slow = slow->next;
        fast = fast->next->next;

        if (slow == fast) {
            return true;
        }
    }

    return false;
}

int main() {
    ListNode a = ListNode(1);
    ListNode b = ListNode(2);
    ListNode c = ListNode(3);

    a.next = &b;
    b.next = &c;

    if (hasCycle(&a)) {
        std::cout << "yes\n";
    }
    else {
        std::cout << "no\n";
    }
    return 0;
}