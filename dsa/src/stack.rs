use std::{fmt::Error, u32};

use crate::node::Node;

pub struct Stack<T> {
    pub top: Option<Box<Node<T>>>,
    pub size: u32,
    pub limit: u32,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack {
            top: None,
            size: 0,
            limit: 100,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn has_room(&self) -> bool {
        self.size < self.limit
    }

    pub fn push(&mut self, value: T) {
        if !self.has_room() {
            panic!("Stack is full");
        }

        self.size += 1;

        if self.is_empty() {
            self.top = Some(Box::new(Node::new(value, None)))
        } else {
            let new_node = Node::new(value, self.top.take());

            self.top = Some(Box::new(new_node));
        }
    }

    pub fn pop(&mut self) -> Result<(), Error> {
        if self.is_empty() {
            return Err(Error);
        } else {
            let popped_node = self.top.take().unwrap();
            self.top = popped_node.next;
            self.size -= 1;
            return Ok(());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push() {
        let mut stack = Stack::new();
        stack.push(1);
        assert_eq!(stack.top.as_ref().unwrap().value, 1);
        assert_eq!(stack.size, 1);

        stack.push(2);
        assert_eq!(stack.top.as_ref().unwrap().value, 2);
        assert_eq!(stack.size, 2);
    }
}
