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

pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut l1 = &l1;
    let mut l2 = &l2;

    let mut head = None;
    let mut rem = 0;
    let mut tail = &mut head;

    while l1.is_some() || l2.is_some() {
        let d1 = match l1 {
            None => 0,
            Some(ref d) => d.val,
        };
        let d2 = match l2 {
            None => 0,
            Some(ref d) => d.val,
        };
        let sum = d1 + d2 + rem;
        let val = sum % 10;
        rem = sum / 10;

        let node = Some(Box::new(ListNode::new(val)));
        match tail {
            None => {
                head = node;
                tail = &mut head;
            }
            Some(n) => {
                n.next = node;
                tail = &mut n.next;
            }
        }

        if let Some(d) = l1 {
            l1 = &d.next;
        }
        if let Some(d) = l2 {
            l2 = &d.next;
        }
    }
    if rem != 0 {
        tail.as_mut().map(|node| node.next = Some(Box::new(ListNode::new(rem))));
    }

    head
}
