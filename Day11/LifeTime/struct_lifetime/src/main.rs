use std::fmt::Display;

// struct with lifetime annotation
struct ImportantExcerpt<'a> {
    part: &'a str,
 }
 
 fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    //-------------------------------------------------------------------------------------

    longest_with_an_annoucement("deepak", "bittu", "sunao");
 }



 fn longest_with_an_annoucement <'a , T> (x:&'a str, y: &'a str,ann: T) -> &'a str 
    // where is used to tell T can only implement Display
    where T:Display,{
        println!("Annoucement {}",ann);
        if (x.len() > y.len()){
            x
        }else{
            y
        }
 
}