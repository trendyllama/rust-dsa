
// we had to add this library reference to the Cargo.toml file
use dsa::{ add, subtract };
use dsa::node::Node;

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long)]
    hello_world: bool,

    #[arg(short, long)]
    add: bool,

    #[arg(short, long)]
    subtract: bool,

    #[arg(short, long)]
    node: bool,

}

fn main() {

    let args = Args::parse();

    if args.hello_world {
        println!("Hello, world!");
    }

    if args.add {
        println!("Adding numbers");
        let result = add(2, 3);
        println!("The sum of 2 and 3 is: {}", result);
    }

    if args.subtract {
        println!("Subtracting numbers");
        let sub_result = subtract(2, 3);
        println!("The difference of 2 and 3 is: {}", sub_result);
    }


    if args.node {
        println!("Creating a node");
        let node = Node::new(5, None);
        println!("Node value: {}", node.value);
    }


}
