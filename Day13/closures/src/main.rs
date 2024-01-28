/*
closure ==> are anonymous functions that you can save in a variable or pass as
arguments to other functions. They are similar to functions, but unlike
functions, closures can capture values from the scope in which they're defined

fn add_one(x:i32) { this is normal function
   x+1
}
 let add_one = |x: i32| x + 1; this is clousre function like above


 // we do not need to annotate params in case of clousres
 let add_one = |x: i32| x + 1;



*/

use std::{thread, time::Duration};

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn give_away(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.get_most_stoked())

        // here ``||self.get_most_stoked()```  is the closure which is function with not params
        // if this function has params then double pipe (||) params appear in these pipe
    }

    fn get_most_stoked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1, // if color is red then this line execute
                ShirtColor::Blue => num_blue += 1, // other wise this
            }
        }

        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    println!("Hello, world!");

    // first set shirt color in inventory

    let store = Inventory {
        shirts: vec![ShirtColor::Red, ShirtColor::Blue, ShirtColor::Red],
    };

    // lets call if use has prefernce
    let user_preference_1 = Some(ShirtColor::Blue);

    let giveaway_1 = store.give_away(user_preference_1);
    println!("user with giveAway1 is {:?}", giveaway_1);

    let user_preference_2 = None;

    let giveaway_2 = store.give_away(user_preference_2);
    println!("user with random shirt color is {:?} ", giveaway_2);

    //-----------------------------------------------------------------------------------

    // closure facts

    let example_closure = |x| x;

    let s = example_closure(String::from("hello")); // compiler will lock the type of params which we call first
                                                    // let n = example_closure(5); // it will fail because compiler locked params type in first run

    //------------------

    // closure borrow immutable variable
    let x = 5;

    let print_immutable = || println!("immutable borrowing {x}");
    print_immutable();

    //closure borrow mutable variable

    let mut y = 5;

    let mut print_mutable = || {
        y += 1;
        println!("mutable borrowing {y}")
    };
    print_mutable();
    println!("{y}");

    // closure taking ownership of variable using 'move' keyword

    let x_key = 11;
    let closure = move || {
        let x = x_key + 1;
        println!("{}", x);
    };

    closure(); // prints "12"
    println!("{}", x_key);

    generte_workout(45, 6)
}

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

//----------------------------------------------------------------------------------------

fn generte_workout(intensity: u32, random_number: u32) {
    let mut cached_result = Cacher::new(|num: u32| {
        println!("calculating slowly....");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity > 25 {
        println!("today do {} pushups ", cached_result.value(intensity));
        println!("next do situps {}", cached_result.value(intensity));
    } else {
        if random_number == 3 {
            print!("oopsss just take break");
        } else {
            println!("run for {} minutes", cached_result.value(intensity));
        }
    }
}
