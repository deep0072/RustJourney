use std::fmt::Display;
fn main() {
    let z;
    println!("Hello, world!");
    let x = String::from("Deepak");
    {
        let y = "ark";
        z = longest_string(&x, y);
    }
    /*
    as y hold static type string which will be held in memory through out the
    program.so only var y will be out of scope not its value
    if y were dynamic string then variable y and as well its value would gone out of the scope
                ||
                \/
     */
    println!("{}", z); // well this prints
    
}

// this function wont run because compiler not sure whether return type will be lived or not
// compiler do not about the return type refrence whose it is.
// fn longest_string(x:&str,   )->&str{
//     if x.len() > y.len(){
//         x
//     }else{
//         y
//     }
// }


// use " &'a str" =>> here 'a define as generic so all params x nd y with 'a also return type.so this tells to compiler  the params and returns have same life time
fn longest_string<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}






