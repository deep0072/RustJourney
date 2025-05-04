fn main() {
    // control flow

    // if condition

    // let condition = false;
    // let z = if condition {7} else {9};
    // println!("condition {}", z); 

    // // loop ==> use keyword loop {} it is divergent

    // let mut counter  = 0;
    // let result =  loop {
    //     counter +=1;
    //     if counter==3 {
    //         break counter; // counter written with break it means  when conditon break it will return counter
    //     }
    // };

    // println!("the result is {}", result); // result will be 3

    // // while loop

    // let mut counter = 3;

    // while counter!=0 {
    //     println!("{} ", counter);

    //     counter-=1;
    // }

    // println!("lift off !");

    // // for loop 

    // let a = [45,7,8,5,3,45];

    // for element in a{
    //     println!("{}", &element);
    // }
    // // for loop in range min..maxvalue where maxvalue is exclusive

    // for i in 5..90{
    //     println!("{}", i);
    // } 

    let mut  line =  vec![1, 2,3,4]; // line is variable 
    line.push(6);
    println!("{:?}", line);

    
    // for i in line.iter(){
    //     //
    //     println!("{}", i*2);
    


    // }
    // println!("{:?}", line);


}
