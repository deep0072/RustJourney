fn main() {
    //variable in rust are immutable by default
    // to  make mutable variable  use keyword mut
    let x = 5;
    let mut y : i128 = 2;
    y = y +9;

    println!("{} , {}",x,y );

    // const variable are also not mutable they are annotated with its type like string, number etc:
    // const cant be  as return in function.
    const MY_AGE : u32 = 45;
    println!("{}", MY_AGE);

    // // scaler data type => which represnt a single value
    // // 1. Integers ==>

    let a = 9822; // decimal
    let b = 0xff; // hex
    let c = 0o77; // Octal
    let d = 0b11110000; // Binary
    let e = b'a'; // Byte (u8 only)
                  // 2. Floating-point numbers
                  // 3. Booleans
    let x = false;
    let y = true;

    // // 4 Character ==> single character in quotes;

    // let g = 'A';
    // let f = '😉';

    // compount types ==> which represent the group of values;

    // 1. tuples ==> fixed size of array holds different type of data types

    // let tup = ("Deepak", 007, (89,7));

    // // destructuring tuples ==> get the value of tuples

    // println!("{}",tup.2.0); // ==> 89
    // let (name, code) = tup;

    // // get value from tuple using dot notation ==> tup.index

    // let (code) = tup.1;

    //------------------------------------------------------------------------------------

    // 2. Array == holds same type of data in it

    let error_codes = [400,300,200];

    // to get the value from array use arr[in dex]
    let success_Code = error_codes[2];

    // to create array containing 0 with length of 8 ==> [zero; lengthofarray]
    let byte = [0;8];
}
