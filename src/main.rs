use exercises::list_nums::{self, ListNode};

mod exercises;

fn to_list(a: &[i32]) -> Option<Box<ListNode>> {
    if a.len() == 0 {
        None
    } else {
        let mut list = Some(Box::new(ListNode::new(a[0])));
        list_nums::set_next(&mut list, to_list(&a[1..]));
        list
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

fn main() {
    // let mut l1 = Some(Box::new(ListNode::new(1)));
    // let l2 = Some(Box::new(ListNode::new(2)));
    // println!("{:?}", l1);
    //
    // errors::set_next(&mut l1, l2);
    //
    // println!("{:?}", l1);


    let l1 = to_list(&[2, 4, 3]);
    let l2 = to_list(&[5, 6, 4]);
    print_list(&l1);
    print!(" + ");
    print_list(&l2);
    print!(" = ");
    print_list(&list_nums::add_two_numbers(l1, l2));

    println!("==============");

    let l1 = to_list(&[9, 9, 9, 9, 9, 9, 9]);
    let l2 = to_list(&[9, 9, 9, 9]);
    print_list(&l1);
    print!(" + ");
    print_list(&l2);
    print!(" = ");
    print_list(&list_nums::add_two_numbers(l1, l2));

    println!("==============");

    let l1 = to_list(&[1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1]);
    let l2 = to_list(&[5,6,4]);
    print_list(&l1);
    print!(" + ");
    print_list(&l2);
    print!(" = ");
    print_list(&list_nums::add_two_numbers(l1, l2));
}
