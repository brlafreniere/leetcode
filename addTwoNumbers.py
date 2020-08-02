# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None

def sum_nodes(n1, n2, tens_digit = 0):
    val1 = 0 if n1 == None else n1.val
    val2 = 0 if n2 == None else n2.val
    sum = val1 + val2 + tens_digit
    if sum >= 10:
        str_val = str(sum)
        ones_digit = int(str_val[1])
        tens_digit = int(str_val[0])
        return (ListNode(ones_digit), tens_digit)
    else:
        return (ListNode(sum), 0)

class Solution:
    def addTwoNumbers(self, l1: ListNode, l2: ListNode) -> ListNode:
        s1, tens_digit = sum_nodes(l1, l2)
        s1_start = s1
        while (l1 and l1.next != None) or (l2 and l2.next != None):
            l1 = l1.next if l1 != None else None
            l2 = l2.next if l2 != None else None
            s1.next, tens_digit = sum_nodes(l1, l2, tens_digit)
            s1 = s1.next

            # we're at the end... make the remainder a node
            if (l1 and l1.next == None) and (l2 and l2.next == None) and tens_digit > 0:
                s1.next = ListNode(tens_digit)

        if ((l1 and l1.next == None) or (l2 and l2.next == None)) and tens_digit > 0:
            s1.next = ListNode(tens_digit)

        return s1_start