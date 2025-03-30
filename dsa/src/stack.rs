
use std::u32;

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

    pub fn push(&mut self, value: T) {

        if self.is_empty() {
            self.top = Some(Box::new(Node::new(value, None)))
        } else {

            let new_node = Node::new(value, self.top.take());

            self.top = Some(Box::new(new_node));

        }
    }






}