use std::rc::Rc;

#[derive(Debug)]
struct Node<T> {
    value: T,
    next: Option<Rc<Node<T>>>,
}

#[test]
fn test_rc() {
    let node = Rc::new(Node {
        value: 1,
        next: None,
    });

    let node2 = Rc::new(Node {
        value: 2,
        next: Some(Rc::clone(&node)),
    });

    let node3 = Rc::new(Node {
        value: 3,
        next: Some(Rc::clone(&node2)),
    });

    println!("Node 1: {:?}", node);
    println!("Node 2: {:?}", node2);
    println!("Node 3: {:?}", node3);
}
