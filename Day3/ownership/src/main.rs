fn main() {
    // - - - - - ownership rules - - - -  - - - -
    // 1 . each value in rust has variable thats called its owner.
    // 2. There can only be one owner at a time.
    // 3. When owner goes out of scope , the value will be dropped.
    println!("Hello, world!");

    let x = 5;
    let y = x; // copy 

    // it just simple copying data nothing else


    //Now let’s look at the String version:
    let s1 = String::from("hello");
    let s2 = s1; // here we are not copying the data
    // println!("{}", s1); // this will give error
     // its kind of shallow copy means . s1 and s2 are the pointer pointing to same content that stored in memory.

    // but here is catch well as s1 assign to s2 then s1 is eliminated so this called move in rust. means s1 content now owned by s2
  
    

    // but if we really wanted to copy then use clone method
    // let s2 = s1.clone();
    // println!("{}", s1); // this will work after cloning;


    // now lets talk about the ownership

    // whenver a variable passed as an argument in function. then params of function will be take the value of that passed variable and 
    // this is called ownership. means passed variable will be moved to function scope and will drop its ownership
    
    let x = String::from("Deepak");
    let y = 67;


    takes_ownership(x); // X's value moves into the function..
                                     
    println!("{}", x); // it wont works because x is no longer valid here

    takes_ownership_of_int(y); 
    /*as integer do not drop its value after passing into function
     because integers are stored in stack and copies of integers are quick to make   
    
    
    
     */ 
    println!("{}", y); // it will works


   


}


fn takes_ownership(some_string:String) {

    println!("{}", some_string);

}
fn takes_ownership_of_int(some_int:i32) {

    println!("{}", some_int);

}