use serde::{Serialize,Deserialize};
#[derive(Serialize,Deserialize)]
struct User{
    username:String,
    password:String
}

impl User {
    fn deserialize_from_string(){

    }

    fn serialize_to_string(){

    }
}

fn main() {
    // let s = User{
    //     username:String::from("deepak"), password:String::from("123")
    // };

    // let res = serde_json::to_string(&s);

    // let des:User = serde_json::from_str(&s).unwrap();
    // println!("{:?}",des);

    // match res {
    //     Ok(s) => println!("{s}"),
    //     Err(e) => println!("error:{e}")
    // }
    // println!("Hello, world!");
    let s1 = String::from("deepak");
    let s2= String::from("kumar");

    {
        let s3 = String::from("ok");
        let s4 = &s3;
        let s5 = &s2;
        longest_string(s3,s5);
        println!("{s3}");
    }
    

}

fn longest_string(s1:&String, s:&String)->&String{
    s1
}