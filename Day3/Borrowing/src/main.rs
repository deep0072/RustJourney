fn main() {
    let s1 = String::from("Deepak");

    let length = get_length(&s1); //& denote the reference

    println!("{}", s1); // it still working because we are passing refrence in function. so this is the borrowing;
}

fn get_length(s:&String) -> usize {
    let size = s.len();
    size
}