fn main() {
    let s1 = String::from("Deepak");

    let length = get_length(&s1); // here we are using only refrence of variable . not actual variable
    println!("{}", length); // output will be 6

    // lets make refrences mutable because refrences are immutable by default
    let mut s2 = String::from("Deepak");
    mutable_ref(&mut s2);
    let s3 = &mut s2;
    println!("{}", s3);

    let s4 = &mut s2; //We also cannot have a mutable reference while we have an immutable one to the same value.

    // println!("{} {}",s3,s4); // this will not work

    println!("{}", s4); // but this will work because s3 already printed at 11th line so s3 scope already ends
}

fn get_length(s: &String) -> usize {
    s.len()
}

fn mutable_ref(some_string: &mut String) {
    some_string.push_str(" Kumar");
    println!("{}", some_string);
}
