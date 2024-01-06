/*
there are two type of error
//1 Unrecoverable errors ==> are always symptoms of bugs, like 
trying to access a location beyond the end of an array, and so we want to immediately stop the program.

// 2. recoverable error, such as a file not found error, we most likely just 

3 . 
want to report the problem to the user and retry the operation */

// panic! => panic macro  will print a failure message, unwind ( which means Rust walks back up the stack and cleans up the data from each function it encounters. ), 
//clean up the stack, and quit

// we need to abort program witout clean up. 
// so we need to do add some config in toml file  ```panic = 'abort'`

use std::{fs::File, io::{self, Read,ErrorKind}};


fn main() {
    // panic!("crash and burn");
    let vec = vec![45,5,6,6,6];
    println!("{:?}", vec[90]); // this is unrecoverable error. vector index out of bound


    // #######################################################################################
    // Recoverable error 

   

    //-------------------------------------------------------------------------------------------
    let file_result = File::open("./newFile.txt");
       //The return type of File::open is a Result<T, E> enum
    

    // lets handle T is success and E is error 
    match file_result {
        Ok(file)=>println!("{:?}",file), // if file exist 
        Err(error)=>panic!("file not exist {:?}",error) // if not then this panic 

    };
    //-------------------------------------------------------------------------------

    // if error comes then do some thing here 
    //if file not exist then try to create file 
   
    let file_result_1 = File::open("./newFile.txt");



    match file_result_1 {
        Ok(file)=>file,
        Err(error) =>match error.kind() {
            ErrorKind::NotFound => match File::create("newFile.txt"){
                Ok(fc) =>fc,
                Err(error) => panic!("some error to create file"),
            },
            other_error=>{
                panic!("problem opening  file");
            }


        }
    };
    // --------------------------------------------------------------------------------------
    // to handle error we can use unwrap

    let new_file = File::open("newFile.txt").unwrap(); // it will panic if file not found and vice versa.

    // can use expect to show expected msg incase of file not  found
    let new_file = File::open("new_File.txt").expect("failed to read");
    //-----------------------------------------------------------------
    // the function given below also handle error 
    read_from_file();



    }





    fn read_from_file()->Result<String,io::Error> {
        // here '?' is used if file found then it will store into my txt file 
        // other wise give error
        let mut my_txt_file = File::open("newFile.txt")?;

        let mut s = String::new();
        // my_txt_file.read_to_string(&mut s)?;

        
        // Ok(s)
        // we can write in one line
        let mut my_txt_file = File::open("newFile.txt")?.read_to_string(&mut s)?;
        Ok(s)

    }

   


