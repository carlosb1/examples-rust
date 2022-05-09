use std::ops;

pub struct Node {
    field: u8,
}

pub fn destroy(node: &Node) {}

impl ops::Add<Node> for Node {
    type Output = Node;
    fn add(self, _add: Node) -> Node {
        Node { field: 1 }
    }
}

pub fn main() {
    println!("hello world");
}
