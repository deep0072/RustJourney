// Collections == >
/**
collection allow
us to store multiple unlike value in array and tuples. because collections are store in to
heap. means size of collection could grow and shrink easily

examples ==> vector, hashmap, string


*/

fn main() {
    //########################################################################################################3
    println!("Hello, world!");
    // Vector is collection type can hold multiple values

    let mut v: Vec<i32> = Vec::new(); // intialize empty vector

    // add iteme in vector using push
    v.push(4);
    v.push(5);
    v.push(6);
    v.push(7);

    // also let intialize the non empty vector using "vec!" ! is exclamantion

    let mut v2 = vec![78, 56, 4, 3, 4, 3];

    // access value of vector using &v[index]

    let mut third_value = &v2[2];

    println!("{}", third_value);

    // using .get(index) method which do not give error if index is out of the bound

    let fourth = v.get(3);

    match fourth {
        Some(fourth) => println!("here is your data at index 4 {}", fourth),
        None => println!("No data found"),
    }

    //----------------------------------------------------------------
    // loop through vector

    for i in &mut v2 {
        *i += 80;        // here * is derefernce operator  used to access the values that i points to. 
        // we are taking reference so original vector also override .
    }

    println!("{:?}", v2); // [158, 136, 84, 83, 84, 83] here each element of vector increased by 80

    // ---------------------------------------------------------------------

    // lets store multiple types of data into vector using enum

    enum SpreadSheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let vector4 = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Float(56.4),
        SpreadSheetCell::Text(String::from("Hello")),
    ];

    let vector4_text = vector4.get(2);

    match vector4_text {
        Some(SpreadSheetCell::Text(i)) => println!("yeah this is Text"),
        _ => println!("no this is something else"),
    };


    // #######################################################################################################

    // String ==> 

    // create new empty string  using String::new()

    let str1 = String::new();

    let data = "initial contents";

    let s = data.to_string(); // to_string() method
    // the method also works on a literal directly:
    let s = String::from("hello");

    // ----------------------------------------------------
    // Updating string
     // Appending to a String with push_str and push

     let mut s = String::from("foo");
     s.push_str("deppak"); // push_str() is used to append string slice
     s.push('!'); // push() only append single character
     println!(
        "{}", s
     );

     // format macro-> 
     let s2 = String::from("hello");
     let s3 = String::from("Deepak");

    let s4 = format!("{}{}",s2,s3); // concatenate string and without moving variable values
    println!("{}",s4);

    //-------------------------------------------------------------------------------------------
    // indexing in string
    // s2[0] // wont work here. because sometime string char could have more than 1 byte
    // so it need to first index will give only half byte which will not work

    // ###################################################################################################

    // HASHMAP ==> like objects in js and dictonary in python
    // to create hashmap we need bring hashmap in scope from standard library

    use std::collections::HashMap;

    let key1 = String::from("Blue");
    let key2 = String::from("Reds");

    
    let mut scores = HashMap::new(); // intiating new empty hashmap

    // now add key value in hashmap
    scores.insert(&key1, 10);
    scores.insert(&key2, 20);

    // scores.insert(true, String::from("Deepak")); // this insertion not valid. key,value pair should be match will the above one

    // access by value using key

    let value = scores.get(&key1); //  we get value in python using .get() if found return Some value otherwise none 

    //so scores.get(&key1) will return option type
    match  value {
        Some(value)=> println!("found {}", value),
        None =>println!("nothing found")
        
    }

    // Adding a Key and Value Only If a Key Isn’t Present using  entry
    let new_key = String::from("Black");


    // if you dont wanted to overrite the key use the syntax below
    scores.entry(&new_key).or_insert(69);
    scores.entry(&new_key).or_insert(40);
    // this line first check if black key exist or not. if exist then it will do nothing. other wise create new key value 
    // and 149 th wont work


    // print key value pair using for loop

    for (key ,value) in &scores {
        println!("key : {}, value: {}", key,value)
    }
    // --------------------------------------------------------

    // lets create program that will insert only unique key 

    let str1 = "hello world Deeapk world";
    let mut  map = HashMap::new();

    for word in str1.split_whitespace(){
        let count = map.entry(word).or_insert(0);
        *count+=1; // here count is related to specific key not the iteration it self

    }
    println!("{:?}", map);
 
 

}
