use std::fmt::format;


pub trait Summary  {

    fn summarize_author(&self)->String; // it need to be mention in child block

    fn summarize(&self)->String{
        format!("Hi there Read more..{}",self.summarize_author()) // even if not call this still it will run child block
    }
    
}
    
struct NewsArticle {
    Headline:String,
    Location:String,
    Author:String
}

impl Summary for NewsArticle{
    fn summarize_author(&self)->String {
        format!("hi {}", self.Author)
    }

    //lets say if we keep this impl block empty then Summary function output will be the default result that is "Hi there Read more.."
    fn summarize(&self)->String {
        format!("{}, by {}", self.Headline, self.Author)
    }

    // if this function not defined in traits block then we need to define it otherwise not .

}


pub struct Tweet {
    pub username:String,
    pub content:String,
    pub reply:bool,
    pub retweet:bool
}

impl Summary for Tweet {
    fn summarize_author(&self)->String {
        format!("{} by {}", self.content,self.username)
    }
    // summarize function already mention in trait block then it will be override in this block
    fn summarize(&self)->String {
        format!("{}", self.username)
    }
}

// traits as params
pub fn notify(item: &impl Summary){
    println!("Breaking News! {}", item.summarize())
}


fn main() {
    println!("Hello, world!");

    let tweet_struct = Tweet{
        username:String::from("tech_update0"),
        content:String::from("hi there"),
        reply:true,
        retweet:true


    };

    let news_struct = NewsArticle{
        Headline:String::from("check this headline"),
        Location:String::from("delhi"),
        Author:String::from("deep")
    };

//    println!("tweet summary {}", tweet_struct.summarize());
//    println!("News Article summary {}", news_struct.summarize());
   notify(&news_struct);
   println!("{}" , returns_summarize().summarize());


}

// lets create function that return type "impl Summary"

fn returns_summarize() -> impl Summary{
    Tweet {
        username:String::from("techy"),
        content:String::from("techy"),
        reply:true,
        retweet:true
    }

    // this return Summary type data then after that return we can access the Summary function or methods
}
