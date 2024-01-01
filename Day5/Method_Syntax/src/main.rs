#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle{
    fn struct_area(&self) ->u32{
        self.width * self.height
    }

    fn can_hold(&self, other:&Rectangle) ->bool{
        self.height > other.height && self.width > other.width
    }
    // here associative function
    // Self is return Type used for struct Rectangle

    // this function will create struct having both side are same
    fn square(size:u32)->Self{
        Self { width: size, height: size }
    }
}

fn main() {
    println!("Hello, world!");

    //---------------------------------------------------------------------------

    // method syntax
    // similar to functions
    // can have params and return value
    //these are defined within the context of struct
    // and first params is always self
    // we use impl keyword with rectangel struct



    // let use area method in struct

    let rect =Rectangle {
        width: 50,
        height:89
    };

    println!("{:?}", rect);

    println!("area of rectangle is {:?}", rect.struct_area());

    let rect2 =Rectangle {
        width: 30,
        height:40,
    };

    let rect3 =Rectangle {
        width: 300,
        height:400,
    };
    

    // now compare mutiple rectangle with another
    // here we are taking &rect2 it means we are not transferring ownership of rect2 to function

    println!("rect can hold rect2 {}", rect.can_hold(&rect2));
    println!("rect can hold rect3 {}", rect.can_hold(&rect3));

    println!("rect is square if sides are equal {:?}",Rectangle::square(90) );

    //---------------------------------------------------------------------
    /*
    Associated Functions => All functions defined within an impl block are 
    called associated functions because they’re associated with the type named after the impl

    to use associative function we will use ``::name_of_function```  
    
     */

   

}
