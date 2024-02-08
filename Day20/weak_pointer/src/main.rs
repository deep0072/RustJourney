use std::cell::{Ref, RefCell};
use std::rc::{Rc, Weak};
#[derive(Debug)]
struct Node {
    val: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    /*
    Thinking about the relationships another way, a parent node should own its children: if a parent
    node is dropped, its child nodes should be dropped as well. However, a child should not own its
    parent: if we drop a child node, the parent should still exist. This is a case for weak references!
    to get the weak refrences we will use Weak<T> smart pointer

    So instead of Rc<T>, we’ll make the type of parent use Weak<T>, specifically a


    */

    let a = Rc::new(Node {
        val: 45,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&a),
        Rc::weak_count(&a),
    );
    
    {
        let b = Rc::new(Node {
            val: 46,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&a)]),
        });
    
        // here we are first taking parent value using * opretor 
        // and then borrowing mutable weak refrence of parent node using downgrade
        // downgrade only works if child has weak pointer to parent
        *a.parent.borrow_mut() = Rc::downgrade(&b);
        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&b), 
            Rc::weak_count(&b),
        );
        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&a), // count is 2 because b pointing its children to clone of Rc node 
            Rc::weak_count(&a), // count is zero
        );
        println!("Hello, world!");
    }
   

    dbg!(a);
}
