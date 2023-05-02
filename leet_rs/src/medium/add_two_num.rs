// https://leetcode.com/problems/add-two-numbers/

// Input: l1 = [2,4,3], l2 = [5,6,4]
// Output: [7,0,8]
// Explanation: 342 + 465 = 807.

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        ListNode {
            val: val,
            next: None,
        }
    }
}

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut l1 = l1;
    let mut l2 = l2;
    let mut dummy_head = ListNode::new(0);
    let mut current = &mut dummy_head;
    let mut carry = 0;

    while l1.is_some() || l2.is_some() {
        let add_res = match (&l1, &l2) {
            (Some(a), Some(b)) => a.val + b.val,
            (Some(a), None) => a.val,
            (None, Some(b)) => b.val,
            _ => 0,
        } + carry;
        // res = 18..then, sum = 8 carry = 1
        carry = add_res / 10;
        let sum = add_res % 10;

        current.next = Some(Box::new(ListNode::new(sum)));
        current = current.next.as_mut().unwrap();

        l1 = if l1.is_some() { l1.unwrap().next } else { None };
        l2 = if l2.is_some() { l2.unwrap().next } else { None };
    }

    if carry > 0 {
        current.next = Some(Box::new(ListNode::new(carry)));
    }

    dummy_head.next
}

#[cfg(test)]
mod tests {

    use crate::medium::add_two_num::ListNode;

    use super::add_two_numbers;
    #[test]
    fn add_two_sum_test() {
        let mut l1_head = ListNode::new(2);
        let mut l1_2nd_item = ListNode::new(4);
        let l1_tail = ListNode::new(3);
        l1_2nd_item.next = Some(Box::new(l1_tail));
        l1_head.next = Some(Box::new(l1_2nd_item));

        let mut l2_head = ListNode::new(5);
        let mut l2_2nd_item = ListNode::new(6);
        let l2_tail = ListNode::new(4);
        l2_2nd_item.next = Some(Box::new(l2_tail));
        l2_head.next = Some(Box::new(l2_2nd_item));

        let l1 = Some(Box::new(l1_head));
        let l2 = Some(Box::new(l2_head));

        let mut expected_head = ListNode::new(7);
        let mut expected_2nd_item = ListNode::new(0);
        let expected_tail = ListNode::new(8);
        expected_2nd_item.next = Some(Box::new(expected_tail));
        expected_head.next = Some(Box::new(expected_2nd_item));

        let expected = Some(Box::new(expected_head));

        let actual = add_two_numbers(l1, l2);
        assert_eq!(expected, actual);
    }
}
