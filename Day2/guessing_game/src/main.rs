use colored::Colorize;
use rand::Rng; // here rand is used to  generate random number
use std::cmp::Ordering;
use std::io; // importing io to get the input of the user

fn main() {
    let secret_number = rand::thread_rng().gen_range(0..100);
    loop {
        println!("secret number is {}", secret_number);

        let mut guess = String::new();
        println!("guess number");

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

       

        // first trim the space from string ==> then parser will convert string to number
        let guess: u32 = match guess.trim().parse() {
            Ok(num)=>num, 
            Err(_)=>continue, // here if we guess other than any number it will continue not throw any error 

        };

        println!("you guesses {}", guess);

        // match keyword only run up to exact match will not move to next line.

        match guess.cmp(&secret_number) {
            Ordering::Less => println!( "{}","too small".red()),
            Ordering::Greater => println!("{}","too Big".red()),
            Ordering::Equal => {
                println!("{}","you win!".green());
                break;
            }
        }
    }
}
