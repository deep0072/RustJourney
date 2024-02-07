use std::rc::Rc;

enum List {

    // Rc<List> ==> this will create clone of List and pass it to as reference 
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};


fn main() {
    println!("Hello, world!");

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));

    // Rc::clone(&a) ==> this means you are essentially telling Rust 
    // that you want to create another owner of the data pointed to by "a"
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));
}
