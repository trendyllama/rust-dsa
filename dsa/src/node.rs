

pub struct Node<T> {

    pub value: T,
    pub next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(value: T) -> Self {
        Node {
            value,
            next: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_creation() {
        let node = Node::<i32>::new(5);
        assert_eq!(node.value, 5);
        assert!(node.next.is_none());
    }
}