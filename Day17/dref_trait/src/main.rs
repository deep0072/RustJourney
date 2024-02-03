use std::ops::Deref;

struct Mybox <T>(T);


// first create function to intiliase the out custom Mybox (smart pointer)
impl <T> Mybox<T> {
    fn new(x:T) -> Mybox<T>{
        Mybox(x)
    }

}

impl <T> Deref for Mybox<T>{
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
        
    }
}

fn hello(m:&str) {
    println!("hi this is me {}", m);
}



fn main() {
    let x = 6;
    let y = Mybox::new(x);

    println!("Hello, world!");
    println!("Hello, world! {}",x);

    assert_eq!(6,x);
    // assert_eq!(6,y); // it will not work because y holding x memory address that point to actual value
    assert_eq!(6,*y); // this syntax will not work cause of our custom MyBox that is smart pointer this works only on case of Box smart pointer

    // to deref we need to use deref trait
    // assert_eq!(6,*y); // *y ==> *(y.dref())


    // deref Corection ==>convert a reference to a type that implements deref trait into a refrence to another type

    let refernce_to_String = Mybox::new(String::from("Deepak Kumar")); 

    // as refernce_to_String is Mybox<String> type

    // we implement deref function in this code. so compiler use that function when we pass that varibal in to hello function as params
    hello(&refernce_to_String); 

}
