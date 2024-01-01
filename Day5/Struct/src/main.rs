// Structs  = Structs are like tuples hold multiple types of data. Unlike tuples order does not matter in struct.
//  in a struct you’ll name each piece of data so it’s clear what the values mean

#[derive(Debug)]
struct User {
    active: bool,
    user_name: String,
    email: String,
    sign_in_count: u64,
}
struct Dimensions(i32, i32);

struct Rectangle {
    width: u32,
    height: u32,
}



fn main() {
    // now create instance of struct intiate the value
    let mut user1 = User {
        active: true,
        email: String::from("deepak@gmail.com"),
        user_name: String::from("Deepak"),
        sign_in_count: 1,
    };

    // get struct value using dot notation "struct.fieldname"
    println!("{}", user1.email);

    //----------------------------------------

    // update field using dot notation
    //change name. updation only work when instance of sturct is mutable
    user1.user_name = String::from("Puneet");
    println!("{}", user1.user_name);

    let new_user = build_user(String::from("deepaaa@gmail.com"), String::from("punnnn"));
    println!("{}", new_user.user_name);

    //--------------------------------------------------------

    //create new instance of struct from existing instance

    let user3 = User {
        email: String::from("deepppp@gmail.com"),
        user_name: String::from("shaktiman"),
        //.. specifies that the remaining fields not explicitly set should have the same value as the fields in the given instance.
        ..user1 // this syntax should comes to the last
    };

    println!("user3 {}", user3.user_name);

    //---------------------------------------------------------------

    // tuple struct ==> similar to tuples
    // don't have name associated with their fields
    // just type of fields
    // syntax ==> struct Name(type, type)

    let rect = Dimensions(45, 47);
    let rect_area = area(rect);

    //------------------------------------------------------------------------
    // derive traits

    let new_rect = Rectangle {
        width: 46,
        height: 23,
    };

    println!(" {:?}", new_rect);

    println!("area is {}", get_rect_area(&new_rect));


    
}

// construct new instance of User Struct via function

fn build_user(email: String, user_name: String) -> User {
    let user = User {
        active: true,
        sign_in_count: 1,
        email, // field init shorthand syntax.
        user_name,
    };

    user
}

fn area(dimensions: Dimensions) -> Dimensions {
    println!("area is {}", dimensions.0 * dimensions.1);
    dimensions
}

fn get_rect_area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
