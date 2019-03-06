pub struct Node<T> {
    elem: T,
}
impl<T> Node<T> {
    pub fn new(elem: T1 ) -> Node<T> {
        Node{elem: elem}
    }
}


fn main () {
    let node = Node::new(1);
    let node2 = Node::new("h");

    println!("Hello world 1 {:?}", node.elem);
    println!("Hello world 2 {:?}", node2.elem);
}

