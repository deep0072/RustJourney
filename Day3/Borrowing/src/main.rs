
// Copy
// Rules of borrowing
// 1. There can me many immutable references at the same time
// 2. There can be only one mutable reference  at a time
// 3. If there is a mutable reference , you can’t have another immutable reference either.

// This to avoid any data races/inconsistent behaviour
// If someone makes an immutable reference , they don’t expect the value to change suddenly
// If more than one mutable references happen, there is a possibility of a data race and synchronization issues



fn main() {
    let s1 = String::from("Deepak");

    let length = get_length(&s1); //& denote the reference

    println!("{}", s1); // it still working because we are passing refrence in function. so this is the borrowing;
}

fn get_length(s:&String) -> usize {
    let size = s.len();
    size
}