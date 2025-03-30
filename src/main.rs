
// we had to add this library reference to the Cargo.toml file
use dsa::{ add, subtract };
use dsa::node::Node;

fn main() {

    let node = Node::new(5);

    println!("Node value: {}", node.value);

    let result = add(2, 3);
    let sub_result = subtract(2, 3);

    println!("The sum of 2 and 3 is: {}", result);
    println!("The difference of 2 and 3 is: {}", sub_result);
    println!("Hello, world!");
}
