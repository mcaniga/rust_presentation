use std::rc::Rc;

enum NodeType {
    Node(i32, Rc<NodeType>),
    Nil,
}

fn run_shared_node_demo() {
    /*
    b c
    | |
    ˇ ˇ
     a
    */

    let a = Rc::new(NodeType::Node(5, Rc::new(NodeType::Nil)));
    println!(
        "Node 'a' was created, reference count to 'a' = {}",
        Rc::strong_count(&a)
    );
    {
        let b = NodeType::Node(3, Rc::clone(&a)); // cloning only reference, not entire object - cheap
        println!(
            "Node 'b' points to 'a' , reference count to 'a' = {}",
            Rc::strong_count(&a)
        );
        {
            let c = NodeType::Node(4, Rc::clone(&a));
            println!(
                "Node 'c' points to 'a' , reference count to 'a' = {}",
                Rc::strong_count(&a)
            );
        }
        println!(
            "Node 'c' was detached, reference count to 'a' = {}",
            Rc::strong_count(&a)
        )
    }
    println!(
        "Node 'b' was detached, reference count to 'a' = {}",
        Rc::strong_count(&a)
    )
}

pub fn run_rc_demo() {
    run_shared_node_demo();
}
