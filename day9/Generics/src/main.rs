//generics ==> these are used to accept any type of data as params in function

//-------------------------------------------------------------------------------

// this function will only take i32 type vector which is concrete type params
// use generics if you wanted to accept any type of data

fn get_largest(number_list: Vec<i32>) -> i32 {
    let mut largest :i32 = number_list[0];

    for i in number_list {
        if i > largest {
            largest = i;

        }
    }
    println!("largest value {}", largest);


    largest
}

//---------------------------------------------------------------------------

// to accept any type of data use <T> T is type just after function name
// PartialOrd+Copy ==> means any types of value can be compared or copy

fn get_any_largest<T: PartialOrd+Copy>(any_list: &[T])-> &T{
    let mut largest = &any_list[0];
    for item in any_list{
        if item > largest{
            largest = item;
        }
    }

    largest

}

// declare struct to accept any type of variable using generics
struct Point <T>{
    X:T,
    Y:T
}


// <T> after impl used to indicate Point struct  is generic type not concrete type 
impl <T> Point<T>{
    fn x(&self) -> &T{
        &self.X

    }
}

impl Point<f32> {
    fn y(&self) -> f32{
        self.Y
    }
}

fn main() {
    println!("Hello, world!");

    let number_list = vec![87,4,5,2,555,33,11];
    let char_list = vec!['h','d','t','u'];


    
    // let largest_number = get_largest(number_list); // as this function only accept number type vector
    // println!("largest_number is {}", largest_number);

    let any_largest_value = get_any_largest(&char_list);
    println!("largest_number is {}", any_largest_value);

    //------------------------------------------------------------------------------
    // example of struct with generic. so same struct can accept any type of value 

    let struct_1 = Point{X:5, Y:6};
    // get function of struct
    println! ("struct_1 x {}",struct_1.x()); // here only x function will be called. becuase y function only accept float type
    //--------------------------------------
    let struct_2 = Point{X:5.0, Y:6.0};
    println!("struct_2 x {}", struct_2.x()); // here x and y both called x can handle any value
    println!("struct_2 y {}", struct_2.y()); // y already takes float value 


    // let s play with mix_struct
    let mix_struct = Mix_Struct {X:5,Y:5.0}; // X,Y is T,U respectively
    let Other_Struct = Mix_Struct {X:"Deepak",Y:'V'}; // here is X,Y is V,W respectively

    let output = mix_struct.mix_up(Other_Struct); // its create struct with type  T,W
    println!("mixup value is {}", output); // spit the output here W type
   
   

    
    
   



    

}

 




// new mix struct

struct Mix_Struct <T,U>{
    X:T,
    Y:U
}

impl<T,U> Mix_Struct<T,U> {
    fn mix_up<V,W> (self,other:Mix_Struct<V,W>) ->W{
        let output =Mix_Struct{
            X:self.X,
            Y:other.Y
        };

        output.Y

        
    }
}