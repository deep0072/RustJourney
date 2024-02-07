// Cell ==>  when you need to have interior mutability, which allows you to mutate data even 
//though the reference to the data is immutable. This is in contrast to Cell<T>, which only 
//provides interior mutability for 
//certain types of data, like integers and booleans

//----------------------------------------------------------------------------------
// use std::cell::Cell;
// #[derive(Debug)]
// struct Node<'a> {
//     val:Cell<i32>, // Cell allow to mutate the value or copy the value but not allow the shared refrences
//     adjacent:Vec<&'a Node<'a>>
// }

// fn add_one(node: &Node){
//     let curr = node.val.get(); // first get the value
//     node.val.set(curr+1); // then set the value

//     for adj in node.adjacent.iter(){
//         add_one(&adj)
//     }
// }


// fn main() {
//     let a = Node {
//         val:Cell::new(5), adjacent:vec![]
//     };
//     let b = Node {
//         val:Cell::new(11), adjacent:vec![&a]
//     };
//     let c = Node {
//         val:Cell::new(17), adjacent:vec![&b]
//     };
//     add_one(&b);
 
//     dbg!(&c);
// }

//-------------------------------------------------------------------------------------------

// Refcell ==> similar to Cell but they also mutate those refrence which do not support copy trait
// like String and Struct. means refrence can be mutated whihc refer to immutable variable that data 
// which do not follow any kind of copy trait


use std::cell::RefCell;
#[derive(Debug)]
struct Node<'a> {
    val:RefCell<String>, // Cell allow to mutate the value or copy the value but not allow the shared refrences
    adjacent:Vec<&'a Node<'a>>
}

fn add_one(node: &Node){
    let mut curr = node.val.borrow_mut(); 
    // first get the value if we wanted to mutate then use .borrow_mut()
    curr.push('!'); // then set the value

    for adj in node.adjacent.iter(){
        add_one(&adj)
    }
}


fn main() {
    let a = Node {
        val:RefCell::new(String::from("Deepak")), adjacent:vec![]
    };
    let b = Node {
        val:RefCell::new(String::from("Kumar")), adjacent:vec![&a]
    };
    let c = Node {
        val:RefCell::new(String::from("Bhaaa")), adjacent:vec![&b]
    };
    add_one(&b);
 
    dbg!(&c);
}
