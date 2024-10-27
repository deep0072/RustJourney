// lifetime mostly used to ensure the validity of refrences thorught out the scope
// lifetimes are mostly intersection of any refrences variable

// rules for lifetime

//1. 
// a function with one parameter gets one lifetime by default ==> fn foo <'a>(x: &'a str)
// a function with two params get two seperate lifetime => fn foo <'a, 'b>(x: &'a str,y: &'b str)

// 2. 
// The second rule is that, if there is exactly one input lifetime parameter, that lifetime is assigned to all output 
// lifetime parameters: fn foo<'a>(x: &'a i32) -> &'a i32. means if there is only one params we dont need to assign the lifetime here


// 3.
// The third rule is that, if there are multiple input lifetime parameters, but one of them is &self or 
// &mut self because this is a method, the lifetime of self is
// assigned to all output lifetime parameters. This third rule makes methods much nicer to read and write because fewer symbols are necessary
/*

struct MyStruct<'a>{

}

impl MyStruct<'a> {
    // only self time will be applicable to all params here
    fn something(&'a self,y:&str ) -> &'a str{}
    
}


*/




fn main() {
    println!("Hello, world!");

    let longest_str;

    let str1 = String::from("hello there!");
    
    let str2 = String::from("ok");
    longest_str = longest(&str1,&str2);

    

    println!("longest str: {}", longest_str);
    //------------------------------------------
    // static lifetime 

    let x = 9;
    let result;

    {   let x2 = 15;
        result = x2;
    }


    println!("hello {result}");


}

// fn longest(str1:String,str2:String)->String {
//     if str1.len()> str2.len(){
//         return str1;
//     }else {
//         return str2;
//     }
// }




//-------------------------------------------------------------------
// function with generic lifetime 
//--------------------------------------------------------------------
fn longest<'a>(str1:&'a str,str2:&'a str)->&'a str {
    /*this not output the  error of lifetime
            here 'a is lifetime that tells the 
            lifetime of params in function 
    'a kind of intersection of both params lifetime. */
    if str1.len()> str2.len(){
        return str1;
    }else {
        return str2;
    }
}
//-------------------------------------------------------
// what if we wanted to return first params that is refrence only
//  then we do not need to annotate the lifetime to second params

fn longest1<'a>(x: &'a str, y: &str) -> &'a str {
    x
}


//----------------------------------------

// life time annotation with struct
// means we are telling that struct need to hold refrences by using lifetime



// this wont work because it through error of lifetime here
// means we need to tell the compiler lifetime of refrences
struct MyStruct {
    name: &str

}

// solution for above problem
struct MyStruct2<'a> {
    name: &'a str

}



