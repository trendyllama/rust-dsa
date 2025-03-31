pub struct Node<T> {
    pub value: T,
    pub next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new(value: T, next: Option<Box<Node<T>>>) -> Self {
        if next.is_none() {
            return Node { value, next: None };
        }

        Node { value, next }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_creation() {
        let node = Node::<i32>::new(5, None);
        assert_eq!(node.value, 5);
        assert!(node.next.is_none());
    }

    #[test]
    fn test_node_with_string() {
        let node = Node::<String>::new("Hello".to_string(), None);
        assert_eq!(node.value, "Hello");
        assert!(node.next.is_none());
    }
}
