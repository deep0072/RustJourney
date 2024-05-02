// Enums ==> enums give you a way of saying a value is one of a possible set of values. For example, we may want to say that Rectangle
//  is one of a set of possible shapes that also includes Circle and Triangle
// enums is used to store different variants of something.
// like loki variants in multiverse

// one of the example is Ip addresses could be ipv4 and ipv6

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

// struct IpAddr {
//     kind:IpAddrKind,
//     address:String
// }
fn main() {
    // create instance of enum
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    // let say there is route function we wanted to pass enum type for example v4 as params
    // router(IpAddrKind::V4)

    //put data into enum variant

    // enum IpAddr {
    //     V4(String),
    //     V6(String)

    // }

    // let home = IpAddr::V4(String::from("http://127.0.0.1:4000"));

    //-----------------------------------------------------------
    // each variant can have different types and amounts of associated data

    /*

    enum IpAddr {
        V4(u8,u8,u8,u8),
        V6(String)
    }

    let home = IpAddr::V4(127, 0, 0, 1);

    let loobpack = IpAddr::V6(String::from("::1"));


    */

    //-------------------------------------------------------------------------------
    // each enum variant can have wide variety of types
    /*

     enum Message {
        Quit,
         Move {x:i32, y:i32}, // this is anonymous struct
         Write(String),
        ChangeColor (i32,i32,i32)
     }



    */

    //-----------------------------------------------------------------------------

    // we can define function in enum using impl keyword

    /*
    
        enum Message {
            Quit,
            Move { x: i32, y: i32 }, // this is anonymous struct
            Write(String),
            ChangeColor(i32, i32, i32),
        }

        impl Message {
            fn d0_something(&self){
                println!("hi what i need to do");
            }
        }

        // to use function from enum we need to use one variant of enum
        let m = Message::Quit;
        m.d0_something()
    
     */

    //----------------------------------------------------------------------------
    // Option enum ==> used to handle null value in rust. these enum are included in our program scope by default
    // so we do not need to declare like we declared below  
    // enum Option<T> {
    //     Some(T), // here t meaning it can handle any types of values
    //     None

    // }
    let number = Some(45); // these are the value which exist and held by Some keyword
    let absent_number: Option<i32> = None; // to define none we need to type annotate like we did Option<i32>

    //---------------------------------------------------------------------------------------------------------

    // lets add some value

    let x = 34;
    let new_number = Some(90);
    // we can't add Optional type integer to normal integer type

    //so to extract value from optional type integer to integer we need to use unwrap
    //new_number.unwrap_or(0) ==> this will extract value from new_number otherwise 0

    let sum = x + new_number.unwrap_or(0);
    println!("{}",sum);

    //-----------------------------------------------------------------------
    // match expression

    value_in_cents(Coin::Rupee(Country::India));

    //---------------------------------------------------------

    // match expression with Option enum

    let five = Some(5); // pass optional integer
    plus_one(None);
  
    
    // well this function goes to 2nd match condition if value is integer other match with None
    fn plus_one(x: Option<i32>)->Option<i32> {
        match x { // first check if params is none
            None=>{
                println!("null value");
                None
            }, // None will work only if there will be absense of number
            Some(i)=>{
                println!("{}", i);
                Some(1+i)
            }, // if param is not none then it will plus to one
        }

    }
    //----------------------------------------------------------------------
    // here in this match expression
    // "new_plus_one_with_value_3" function will accept only value 3 while other will pass to None _=> used to accept multiple value which we do not want



    let three_value = Some(3);
    let four_value = Some(4);
    new_plus_one_with_value_3(four_value);

    fn new_plus_one_with_value_3(x:Option<i32>) ->Option<i32> {
        match x {
            Some(3)=>{
                println!("accepting value 3");
                Some(1+3)
            },
            _=>{
                println!("miscellaneous values");None},
        }
   }

   

}

#[derive(Debug)]

enum Country {
    India,
    Usa
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(Country), // here passing another enum to variant 
    Rupee(Country)
}


fn value_in_cents(coin:Coin)->u8{
    match coin {
        Coin::Penny =>{
            println!("lucky penny");
            1
        },

        Coin::Dime=>10,
        Coin::Nickel=>5,
        Coin::Quarter(state)=>{
            println!("currency from {:?} !", state);
            25
        },
        Coin::Rupee(country)=>{
            println!("currency from {:?} !", country);
            10
        }
    }
}

