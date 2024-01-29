// create custom iterator

struct Counter{
    count:u32
}

impl Counter {
    fn new()->Counter {
        Counter{count:0}
    }
}

impl Iterator for Counter {
    type Item=u32;

    // &mut self ==> Here we are refrencing to the Counter Struct which is mutable
    fn next(&mut self) ->Option<Self::Item>{
        if self.count<5{
            self.count+=1; // here we are trying to increment the Struct element by 1
            Some(self.count) // its Option return type
        }else{
            None
        }
    }
}



fn main() {
    println!("Hello, world!");
}


#[test]

fn calling_next_directly(){
    let mut counter = Counter::new(); // intialise the counter using new function

    // here .next() is our iterator
    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));

}