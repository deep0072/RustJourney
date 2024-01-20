/*
1. simple cli tool that find string into file when we type in terminal*/

use std::env::args; // used to read command line args

use std::process;

use mini_grep::Config;

fn main() {
    let args: Vec<String> = args().collect();

    // now store command line args  in variable

    let file_config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);

        //process::exit function will stop the program immediately and return the
        //number that was passed as the exit status code
        process::exit(1)
    });

    // open file and read its content using read_to_string
    if let Err(e) = mini_grep::run(file_config) {
        println!("Application error {e}");
        process::exit(1);
    }
}


