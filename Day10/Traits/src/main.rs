// traits ==> are the kind of interfaces
// means common  functions structure that can be 
// used at different places

use std::fmt::{Display, Debug};

trait Summary {

    fn get_author(&self) -> &str;


    // this function will be called by any struct by default
    fn summarize(&self) -> String{
        
        format!("{} read more...", self.get_author())

    }
    
}

struct NewsArticle{
    pub headline:String,
    pub location: String,
    pub author: String,
    pub content: String
}

impl Summary for NewsArticle {

    // if summarize function not defined with its body then it need to be define here

    // fn summarize(&self) -> String {
    //     format!("author: {}, headline: {}, news: {}", self.author,self.headline,self.content)
    // }

    //-------------------------------------

    fn get_author(&self) -> &str {

        self.author.as_str()
        // let news_summary = format!("author: {}, headline: {}, news: {}", self.author,self.headline,self.content);

        // news_summary.as_str()


        
    }
}

trait  MyTrait {
    fn demo(&self) {
        println!("hi from demo trait")
    }
    
}


struct Tweet{
    username: String,
    content:String,
    reply: bool,
    retweet:bool
}

// implement trait for 

impl Summary for Tweet {
    // need to add same function we used in Summary trait
    // if summarize function not defined with its body then it need to be define here
    // otherwise summarize trait's function will be called by default
    // fn summarize(&self) -> String {
    //     format!("tweet:{}, {}", self.username,self.content)
        
    // }

    //------------------------------------------

    fn get_author(&self) -> &str {
        // let  tweet = format!("tweet:{}, {}", self.username,self.content);
        self.username.as_str()
    }

    //summarize function called by default which call the get_author function
}

impl MyTrait for Tweet{}



fn main() {
    println!("Hello, world!");

    

    let tweet = Tweet {
        username:String::from("tech_update0"),
        content: String::from("hi am  learning rust "),
        reply:false,
        retweet:false
    };
    let tweet2 = Tweet {
        username:String::from("tech_update02 output with implementation of trait bound"),
        content: String::from("output with implementation of trait bound"),
        reply:false,
        retweet:false
    };
    let news_article = NewsArticle {
        headline:String::from("lorence bishnoi sbke lore lgade"),
        location: String::from("hi am  learning rust "),
        author:String::from("deepak"),
        content:String::from("lorence bishnoi hua khatarnak")
    };

    // as aggregator function accept summary trait 
    // if we pass tweet struct then it accept cause tweet struct implement Summary trait

    // aggregator(tweet);
    // aggregator(news_article);
    // get_news(tweet2);

    //-----mixing multiple traited bound struct
    mix_up_news(&tweet, &news_article);
    mixup_trait(&tweet);

    //-----------------------------

    /*
     this wont work here because it doesnot implement Mytrait
    
     */
    // mixup_trait(&news_article);




}

// now lets create function the aggregate the info from NewsArticle as well as from tweeter

// source: impl Summary ==> traits as parameter passed in aggregator

fn aggregator(source: impl Summary) {
    // impl Summary =>> means we are implementing Summary trait

    // Tweet struct uses Summary trait


    println!("{}", source.summarize());

}

//--------------------------trait bound-------------
// fn get_news<T>(source:T){
//     println!("{}", source.summarize())

//     // as in this function we are trying to call summarize()
//     // but we do not know whats the T here.
//     // we need to use bound where we can tell that T is only refer to Summary Trait
// }
fn get_news<T: Summary>(source:T){
    println!("{}", source.summarize())

    // as in this function we are trying to call summarize()
    // but we do not know whats the T here.
    // we need to use bound where we can tell that T is only refer to Summary Trait
}

// pass multiple argument with traits

/*  but this wont work thouh botht implements Summary trait
but each struct each is differnet instance of struct it self

*/

// fn mix_up_news<T:Summary>(source1: &T, source2: &T){
//     println!("source1 {}, source2 {}", source1.summarize(), source2.summarize())

// }


fn mix_up_news(source1: &impl Summary, source2: &impl Summary){
    println!("source1 {}, source2 {}", source1.summarize(), source2.summarize())

}

// ---------------------------------------------


/*
let say tweet struct uses multiple trait and we wanted to show 
all its trait's function in function 


*/

/*
params are saying that pass those 
parameter who implements both trait  Summary and MyTrait

*/
fn mixup_trait(source: &(impl Summary + MyTrait)) {

    println!("summary trait {}, Mytrait is {:?}", source.summarize(), source.demo())

}


//-----------------------------------------

// we can also use which generics implements trait bound

// fn some_function<T,U>(t: &T, u:&U)  -> i32 
// where  T:Display + Clone, U: Clone + Debug {

// }



