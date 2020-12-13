#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
}

pub fn set_next(list: &mut Option<Box<ListNode>>, next: Option<Box<ListNode>>) {
    if let Some(node) = list {
        node.next = next;
    }
}

pub fn set_val(list: &mut Option<Box<ListNode>>, val: i32) {
    if let Some(node) = list {
        node.val = val;
    }
}

pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut l1 = &l1;
    let mut l2 = &l2;

    let mut head = Some(Box::new(ListNode::new(0)));
    let mut rem = 0;
    let mut curr = &mut head;

    while l1.is_some() || l2.is_some() {
        let d1 = match &l1 {
            Some(d) => d.val,
            None => 0
        };
        let d2 = match &l2 {
            Some(d) => d.val,
            None => 0
        };
        let sum = d1 + d2 + rem;
        let val = sum % 10;
        rem = sum / 10;
        set_val(curr, val);
        if let Some(l) = l1 {
            l1 = &l.next;
        }
        if let Some(l) = l2 {
            l2 = &l.next;
        }
        if l1.is_some() || l2.is_some() {
            set_next(curr, Some(Box::new(ListNode::new(0))));
            curr = &mut curr.as_mut().unwrap().next;
        } else if rem != 0 {
            set_next(curr, Some(Box::new(ListNode::new(1))));
        }
    }

    head
}
