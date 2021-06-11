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

struct Solution;

impl Solution {
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut pre_head = Some(Box::new(ListNode { val: 0, next: head }));
        let mut last = &mut pre_head;
        while let Some(node) = &mut last.as_mut()?.next {
            if node.val == val {
                last.as_mut()?.next = node.next.take();
            } else {
                last = &mut last.as_mut()?.next;
            }
        }
        return pre_head?.next;
    }
}

fn v2l(input: Vec<i32>) -> Option<Box<ListNode>> {
    if input.is_empty() {
        return None;
    }
    let mut root = Some(Box::new(ListNode::new(input[0])));
    let mut node = &mut root;
    for val in input.into_iter().skip(1) {
        let mut n = node.as_mut().unwrap();
        n.next = Some(Box::new(ListNode::new(val)));
        node = &mut n.next;
    }
    root
}

fn main() {
    assert_eq!(
        Solution::remove_elements(v2l(vec![1, 2, 6, 3, 4, 5, 6]), 6),
        v2l(vec![1, 2, 3, 4, 5])
    );
    assert_eq!(
        Solution::remove_elements(v2l(vec![7, 7, 7, 7]), 7),
        v2l(vec![])
    );

    assert_eq!(
        Solution::remove_elements(v2l(vec![1, 2, 2, 1]), 2),
        v2l(vec![1, 1])
    );
}
