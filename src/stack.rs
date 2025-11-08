enum Stack {
    Cons(u32, Box<Stack>),
    Nil,
}

use crate::Stack::*;

impl Stack {
    fn new() -> Stack {
        Nil
    }

    fn push(self, val: u32) -> Stack {
        Cons(val, Box::new(self))
    }

    fn pop(self) -> Stack {
        match self {
            Cons(_, tail) => *tail,
            Nil => Nil,
        }
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
            Nil => "Nil".to_string(),
        }
    }
}

fn main() {
    let mut stack = Stack::new();
    stack = stack.push(10);
    stack = stack.push(20);
    stack = stack.push(30);
    println!("{}", stack.stringify());
    stack = stack.pop();
    println!("{}", stack.stringify());
    stack = stack.pop();
    println!("{}", stack.stringify());
    stack = stack.pop();
    println!("{}", stack.stringify());
    stack = stack.pop();
    println!("{}", stack.stringify());
    stack = stack.push(40);
    println!("{}", stack.stringify());
    stack = stack.push(50);
    println!("{}", stack.stringify());
    stack = stack.push(60);
    println!("{}", stack.stringify());
}
