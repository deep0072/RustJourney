use std::sync::{Arc,Mutex};
use std::thread;

fn main() {

    //------------------------------------------------------------------------------------------------

    // what if mutiple thread wanted to use same data. but data ownership changes as we jump to another thread

    // to get use the shared data or messages among threads Rust provide "MUTEX" 

    // Mutex ==> its abbrivation is Mutual Exclusion
    //it allows each thread to use shared data for given time period

    // how it does
    // first thread give signal
    // then mutex acuire lock.Lock is data structure which keeps track of who currenlty using that data.
    //when particular thread done with the shared data.Mutex will unlock the data to make it freely usable for other threads

    // let m = Mutex::new(5); // hold integer five
    // {
    //     let mut num = m.lock().unwrap(); // this will lock that value for this thread only

    //     // as m.lock() return mutex guaard as smartpointer so we need to use deref to get the actual data of pointer
    //     *num=6;
    //  }

    //  println!("here is the value: {:?}",m);




     // now lets share single varaible within different threads

     // Arc ==>atomic refrence count which is used pnly when data being shared in threads

     // as we are using closures so they in each looped use same counter varaible and move intot multiple threads
     // so to assign multiple owner we use Rc or Arc smart pointer. As Rc is not safe to use so we need to use Arc smart pointer

     let new_counter = Arc::new(Mutex::new(1));

     let mut handles = vec![];

     for _ in 0..10 {
        let new_counter = Arc::clone(&new_counter);
        let handle = thread::spawn(move || {
            let mut num = new_counter.lock().unwrap();

            *num += 1;
        });

        handles.push(handle);
     
    }

    for handle in handles{
        handle.join().unwrap(); // here we are trying to complete each thread using join method 
    }

    // now again lock to use the value from the mutex itself

    
    println!("Hello, world! {}", *new_counter.lock().unwrap());
}
