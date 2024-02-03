trait Vehicle {
    fn drive(&self);
}
// smart pointers ==> these are just refrences that store  addresses of memory that points to data allocated on heap



struct Truck{
    //------------------------------------------------------------------------------------------------------------
    // 2. second use case of Box smart pointer  what if we wanted to define recursive type like struct has field which is also struct
    next_truck: Option<Box<Truck>>,
}

impl Vehicle for Truck{
    fn drive(&self){
        println!("truck is driving")
    }
}

struct MyStruct {
    field1: i32,
    field2: String,
}


fn main() {
    /*
Box pointer ==> 
1. these are used when we are not sure what the exact size of data that we wanted to store on heap

*/
    let t: Box<dyn Vehicle>; // here we are using Box because we do not know the exact size of trait here. 

    t = Box::new(Truck);

    t.drive();





}
