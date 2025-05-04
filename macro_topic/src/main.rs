use std::fmt::{Debug, Display};

macro_rules! generate_functions {
    ($($func_name:ident),*) => {
        $(
            fn $func_name() {
                println!("Hello from {}", stringify!($func_name));
            }
        )*
    };
}


// generate_functions!(foo, bar, baz);

#[derive(Debug)]
struct User{
    username:String,
    age:u32
}

// impl Display for User{
//     fn fmt(&self, f:&mut std::fmt::Formatter)->std::fmt::Result{
//         write!(f, "User {} {}", self.username,self.age)
//     }
// }
// impl Debug for User{
//     fn fmt(&self, f:&mut std::fmt::Formatter)->std::fmt::Result{
//         write!(f, "User {} {}", self.username,self.age)
//     }
// }

generate_functions!(foo, bar, baz);


fn main (){
    let user = User{
        username:String::from("John"),
        age:28
    };

    println!("{:?}",user);
    foo();
    
}