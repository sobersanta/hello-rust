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

fn to_list(a: &[i32]) -> Option<Box<ListNode>> {
    if a.len() == 0 {
        None
    } else {
        Some(Box::new(ListNode::new_full(a[0], to_list(&a[1..]))))
    }
}

fn print_list(list: &Option<Box<ListNode>>) {
    print!("[");
    let mut l = list;
    while let Some(n) = l {
        if l != list {
            print!(",");
        }
        print!("{}", n.val);
        l = &n.next;
    }
    print!("]");
}

pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut l1 = l1;
    let mut l2 = l2;

    let mut head = None;
    let mut rem = 0;
    let mut tail = head.as_mut();

    while l1.is_some() || l2.is_some() {
        let mut sum = rem;
        if let Some(d) = l1 {
            l1 = d.next;
            sum += d.val;
        }
        if let Some(d) = l2 {
            l2 = d.next;
            sum += d.val;
        }

        rem = sum / 10;

        let node = Some(Box::new(ListNode::new(sum % 10)));
        match tail {
            None => {
                head = node;
                tail = head.as_mut();
            }
            Some(n) => {
                n.next = node;
                tail = n.next.as_mut();
            }
        }
    }
    if rem != 0 {
        if let Some(node) = tail {
            node.next = Some(Box::new(ListNode::new(rem)));
        }
    }

    head
}
