use std::rc::Rc;

pub struct Node<T> {
    elem: T,
    next: Option<Rc<Node<T>>>,
    previous: Option<Rc<Node<T>>>,
}
impl<T> Node<T> {
    pub fn new(elem: T, next: Option<Rc<Node<T>>> 
               , previous: Option<Rc<Node<T>>>) -> Node<T> {
        Node{elem: elem, next: next, previous: previous }
    }
    pub fn add(self, to_add: Option<Rc<Node<T>>>) {
        if let Some(next) = self.next.clone() {
            to_add.unwrap().next = self.next.clone();   
        } 
        to_add.unwrap().previous = Some(Rc::new(self)); 
    }
}


fn main () {
    let node = Node::new(1, None, None);
    let ref_node = Rc::new(node);
    let node2 = Node::new(2, Some(ref_node.clone()), Some(ref_node.clone()));


//  println!("Hello world 1 {:?}", node.elem);
//  println!("Hello world 2 {:?}", node2.elem);
}

