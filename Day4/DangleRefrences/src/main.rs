fn main() {
    // let reference_to_nothing = dangle();
    // println!("{}", reference_to_nothing);


    // string slice example
    let s = String::from("hello");

    let len = s.len();

    // [starting_index..ending_index]
    let slice1 = &s[0..2];
    let slice2 = &s[..];
    println!("{} {}", slice1,slice2);

    // slice on array
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
    



}

// fn dangle() -> &String { // Returns a reference to a String
//     let s = String::from("hello"); // Create a String
//     &s // Return a reference to the String but it will not work cause we are returning refrence but actual string scope already end at 7th line
// }







