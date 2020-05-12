struct Solution;

struct ListNode {
    val: u32,
    next: Option<Box<ListNode>>
}

impl ListNode {
    fn new(val: u32) -> Self {
        ListNode{
            val,
            next:None
        }
    }
}

impl Solution {
}

#[test]
fn test() {
}