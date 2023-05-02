// https://leetcode.com/problems/merge-two-sorted-lists/

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut result_head = Box::new(ListNode::new(0));
    let mut current_head = &mut result_head;
    let mut l1_current = list1;
    let mut l2_current = list2;

    while l1_current.is_some() && l2_current.is_some() {
        // tick here, take content and ownership from l1_current and l2_current, leave l1_current = None, l2_current = None,  l1_current and l2_current must be available after this while block
        let mut l1_d = l1_current.take();
        let mut l2_d = l2_current.take();
        if let (Some(mut l1_head), Some(mut l2_head)) = (l1_d, l2_d) {
            if l1_head.val <= l2_head.val {
                l1_current = l1_head.next.take();
                // reassign l2_current to get back it's content
                l2_current = Some(l2_head);
                current_head = current_head.next.get_or_insert(l1_head);
            } else {
                l2_current = l2_head.next.take();
                l1_current = Some(l1_head);
                current_head = current_head.next.get_or_insert(l2_head);
            }
        }
    }
    if l1_current.is_some() {
        current_head.next = l1_current;
    }
    if l2_current.is_some() {
        current_head.next = l2_current;
    }

    return result_head.next;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let mut l1_head = ListNode::new(1);
        let mut l1_2nd_item = ListNode::new(2);
        let l1_tail = ListNode::new(4);
        l1_2nd_item.next = Some(Box::new(l1_tail));
        l1_head.next = Some(Box::new(l1_2nd_item));

        let mut l2_head = ListNode::new(1);
        let mut l2_2nd_item = ListNode::new(3);
        let l2_tail = ListNode::new(4);
        l2_2nd_item.next = Some(Box::new(l2_tail));
        l2_head.next = Some(Box::new(l2_2nd_item));

        let l1 = Some(Box::new(l1_head));
        let l2 = Some(Box::new(l2_head));

        let mut expected_head = ListNode::new(1);
        let mut expected_2nd_item = ListNode::new(1);
        let mut expected_3rd_item = ListNode::new(1);
        let mut expected_4th_item = ListNode::new(3);
        let mut expected_5th_item = ListNode::new(4);
        let expected_tail = ListNode::new(4);
        expected_5th_item.next = Some(Box::new(expected_tail));
        expected_4th_item.next = Some(Box::new(expected_5th_item));
        expected_3rd_item.next = Some(Box::new(expected_4th_item));
        expected_2nd_item.next = Some(Box::new(expected_3rd_item));
        expected_head.next = Some(Box::new(expected_2nd_item));

        let expected = Some(Box::new(expected_head));

        let actual = merge_two_lists(l1, l2);
        assert_eq!(expected, actual);
    }
}
