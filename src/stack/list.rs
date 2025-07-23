use crate::{list::node::Node, stack::stack::Stack};

pub struct ListStack<T> {
    head: Option<Box<Node<T>>>,
    len: usize,
}

impl<T> Stack<T> for ListStack<T> {
    fn push(&mut self, value: T) {
        let new_node = Box::new(Node::new(value, self.head.take()));
        self.head = Some(new_node);
        self.len += 1;
    }

    fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.len -= 1;
            self.head = node.next;
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

impl<T> ListStack<T> {
    pub fn new() -> Self {
        ListStack { head: None, len: 0 }
    }
}
