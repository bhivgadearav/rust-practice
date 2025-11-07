enum Item {
    Cons(u32, Box<Item>),
    Nil,
}

use crate::Item::*;

impl Item {
    fn new() -> Item {
        Nil
    }

    fn prepend(self, val: u32) -> Item {
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
    let mut list = Item::new();
    list = list.prepend(10);
    list = list.prepend(20);
    list = list.prepend(30);

    let len = list.len();
    println!("Length: {}", len);
    println!("{}", list.stringify());
}
