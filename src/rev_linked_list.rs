enum LinkedList {
    Cons(u32, Box<LinkedList>),
    Nil,
}

use crate::LinkedList::*;

impl LinkedList {
    fn new() -> LinkedList {
        Nil
    }

    fn prepend(self, val: u32) -> LinkedList {
        Cons(val, Box::new(self))
    }

    fn len(&self) -> u32 {
        match *self {
            Cons(_, ref tail) => 1 + tail.len(),
            Nil => 0,
        }
    }

    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                format!("{}, {}", head, tail.stringify())
            }
            Nil => {
                format!("Nil")
            }
        }
    }
}

fn main() {
    let mut list = LinkedList::new();
    list = list.prepend(10);
    list = list.prepend(20);
    list = list.prepend(30);

    let len = list.len();
    println!("Length: {}", len);
    println!("{}", list.stringify());
}
