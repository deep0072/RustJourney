// package ==> when use cargo new then package created

// package stores crate
//crates==> it can be a library or it could be any executable file
// crates example ==> main.rs is out crate root means it should have binary exucutable which is crate
// crates contain module and module contain code eg. Authentication module could have login code etc,

// workspaces contain lots of packages 

//---------------------------------------------------
//package ==> as our project name is module_system


// create module using keyword mod

mod front_of_house;

// we can use 'use' keyword to make the mode in global scope also call absolute path
use crate::front_of_house::hosting;
// after using 'use' we don't need to call this bigger line front_of_house::hosting::add_to_wait_list()

//---------------------------------------------------------------------------------
// use self::front_of_house::hosting;
// // here self refrencing to current module means we will able to access of current module

/* lets say you want to import write and io module from std
1. use  std::io;
2.  use std::io::Write;

 we can import this like

3. use std::io::{self,Write};
*/




fn main() {

    // lets call add_to_wait_list() function using relative path
    // front_of_house::hosting::add_to_wait_list(); // :: used to access child mod or function

    hosting::add_to_wait_list()

}






