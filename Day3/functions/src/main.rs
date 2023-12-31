fn main() {
    println!("Hello, world!");
    let z = my_function(34,67);
    println!("return  value from function is {}", z)
}

// function accepting params with type annotation
// fn my_function(x:i32, y:i32) {
//     println!("hi this is function");
//     println!("hi this is function {} {}", x, y);
// }

// statement ==> which preform some action but do not return value
// expression ==> which evaluate some values and return in function
// to return in function we will use return keyword in function also need to mention return type

// fn name_of_function(x:i32)  -> (return type) { return  x ;}

fn my_function(x:i32, y:i32)  -> i32 {

    return x + y; //the last line of function is implicitly return

    // to return the we do not need to use return keyword here also remove semicolon at the end of line

    x+y

}