use std::{fs, env};

use std::error::Error;
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case:bool,
}

impl Config {
    pub fn build(mut args: env::Args) -> Result<Config, &'static str> {
        // 'string is life time
        // if args.len() < 3 {
        //     return Err("not enough arguments");
        // }

        // let query = args[1].clone(); // and our main args start from 1 because first value is binary
        // let file_path = args[2].clone();

        //------------------------------------optimised Approach using iterator approach-----------------------------------------

        args.next(); // just skip first arg that is file part of out program

        let query = match args.next(){
            Some(arg)=>arg,
            None=>return Err("no query mentioned here"),
        };
        let file_path = match args.next(){
            Some(arg)=>arg,
            None=>return Err("no file path mentioned here"),
        };


        let ignore_case = env::var("IGNORE_CASE").is_ok(); // it will read value from env variable 
    
        println!("searching for {}", query);
        println!("In file {}", file_path);

        Ok(Config { query, file_path, ignore_case}) // here Ok depict the success because we use Result num type ere
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.file_path)?;
    

    let results = if config.ignore_case {
        case_sensitive_search(&config.query, &content)
    } else {
        search(&config.query, &content)
    };

    
    for line in results{
        println!("matched lines are : {:?}",line);

    };
   

    // println!("result: {:?}", result);
    
    Ok(()) // it will not return anything. we are running this function just for side effects like error
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    // let mut results = Vec::new();
  
    // for line in content.lines() {
   
    //     if line.contains(&query) {
    //         results.push(line);
    //     }
    // }
    // results

    // here optimised version of above approach


    content
        .lines()
        .filter(|line| line.contains(query)) // this filter out the string which contain the quey
        .collect() // convert iterator into collection means crammed each value into line


}
pub fn case_sensitive_search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    let mut query = query.to_lowercase();
    for line in content.lines() {
       
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}

// #[cfg(test)]
// mod tests {
//     use super::*; // import all file from test parent module

//     #[test]
//     fn one_result() {
//         let query = "duct";
//         let contents = "\
// Rust:
// safe, fast, productive.
// Pick three.";

//         assert_eq!(vec!["safe, fast, productive."], search(query, contents));
//     }
// }

