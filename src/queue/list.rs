use std::fmt::Debug;

use crate::{list::node::Node, queue::queue::Queue};

pub struct ListQueue<T> {
    head: Option<Box<Node<T>>>,
    len: usize,
}

impl<T> Queue<T> for ListQueue<T> {
    fn push(&mut self, value: T) {
        let new_node = Box::new(Node { value, next: None });

        if self.head.is_none() {
            self.head = Some(new_node);
        } else {
            let mut current = self.head.as_mut().unwrap();
            while let Some(ref mut next) = current.next {
                current = next;
            }
            current.next = Some(new_node);
        }

        self.len += 1;
    }

    fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            self.len -= 1;
            node.value
        })
    }

    fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.value)
    }

    fn len(&self) -> usize {
        self.len
    }
}

impl<T: Debug> ListQueue<T> {
    pub fn new() -> Self {
        ListQueue { head: None, len: 0 }
    }

    pub fn println(&self) {
        let mut curr = &self.head;
        print!("[");
        while let Some(node) = curr {
            print!("{:?}", node.value);
            curr = &node.next;
            if curr.is_some() {
                print!(", ");
            }
        }
        println!("]");
    }
}
