// drop trait =>> it is mostly used to free up the memory,e files or network connections
// it is called by compiler by default if we just mention drop function

struct CustomSmartPointer{
    data:String
}

impl Drop for  CustomSmartPointer{
    fn drop(&mut self){
        println!("droppping from custome pointer {}", self.data);
    }
}

fn main() {
    println!("Hello, world!");

    let first_Smart_pointer = CustomSmartPointer{data:String::from("first smartpointer")};
    drop(first_Smart_pointer);

    let second_Smart_pointer = CustomSmartPointer{data:String::from("second smartpointer")};
    // after above line drop function will be called by compiler
    // variable dropping occure in reverse order. 'second_Smart_pointer' will be dropped first 

    //------------------------------------------------------------------------------------------------------

    // lets say for some reason we wanted to delete 'first_Smart_pointer' then we need to use  std::mem::drop
  
}
