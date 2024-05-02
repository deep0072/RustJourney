// Option enum is mostly used to find whether any value is null or not.
// means value present or not.
// it absence is checked by pattern matching
fn main( ){
    println!("Hello, world!");
    let my_string = find_first(String::from("ppple"));
    match my_string{
        Some(index) => println!("found it {}",index),
        None => println!("nothing found here")
    }
}


fn find_first(s:String)-> Option<i32>{
    for (index, char) in s.chars().enumerate(){
        if char=='a'{
            return Some(index as i32)
        }
    }
    return  None;
}