struct Solution;

struct ListNode {
    value: u32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(value: u32) -> Self {
        ListNode { value, next: None }
    }
}

impl Solution {
    pub fn add_two_numbers(
        first: Option<Box<ListNode>>,
        last: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let (mut first, mut last) = (first, last);
        let mut head = Some(Box::new(ListNode::new(0)));
        let mut tail = &mut head;
        let (mut first_end, mut last_end, mut overflow) = (false, false, false);
        loop {
            let first_node = match first {
                Some(node) => {
                    first = node.next;
                    node.value
                }
                None => {
                    first_end = true;
                    0
                }
            };
            let last_node = match last {
                Some(node) => {
                    last = node.next;
                    node.value
                }
                None => {
                    last_end = true;
                    0
                }
            };
            if first_end && last_end && !overflow {
                return head.unwrap().next;
            }
            let sum = first_node + last_node + if overflow { 1 } else { 0 };
            let sum = if sum >= 10 {
                overflow = true;
                sum - 10
            } else {
                overflow = false;
                sum
            };
            tail.as_mut().unwrap().next = Some(Box::new(ListNode::new(sum)));
            tail = &mut tail.as_mut().unwrap().next
        }
        None
    }
}

#[test]
fn test() {}
