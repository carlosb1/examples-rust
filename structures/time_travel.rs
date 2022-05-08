use std::ops;

pub struct<T> Node {
    field: T
}

pub fn destroy(node: &Node) {
    
}

impl ops::Add<Node> for Node {
    type Output = Node;
    fn add (self, _add: Node) -> Node {
        Node {}
    }
}

