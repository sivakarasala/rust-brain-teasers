use std::cell::RefCell;
use std::rc::Rc;

type Link = Option<Rc<RefCell<Node>>>;

// #[derive(Debug)]
struct Node {
    elem: i32,
    next: Link,
}

use std::fmt;
impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "elem: {}", self.elem)
    }
}

fn main() {
    let mut head = Some(Rc::new(
        RefCell::new(Node{ elem: 1, next: None })
    ));
    head
        .as_mut()
        .unwrap()
        .borrow_mut()
        .next = Some(Rc::new(RefCell::new(
            Node { elem: 2, next: head.clone() }
        )));

    println!("{:?}", head);
}
