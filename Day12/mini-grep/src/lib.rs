use std::fs;

use std::error::Error;
pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        // 'string is life time
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone(); // and our main args start from 1 because first value is binary
        let file_path = args[2].clone();
        println!("searching for {}", query);
        println!("In file {}", file_path);

        Ok(Config { query, file_path }) // here Ok depict the success because we use Result num type ere
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.file_path)?;
    for line in search(&config.query,&content){
        println!("matched lines are : {:?}",line);

    };
   

    // println!("result: {:?}", result);
    
    Ok(()) // it will not return anything. we are running this function just for side effects like error
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in content.lines() {
        println!("line:  {}", line);
        if line.contains(query) {
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