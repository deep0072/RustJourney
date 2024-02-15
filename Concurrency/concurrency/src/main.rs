use std::thread;
use std::time::Duration;

use std ::sync::mpsc;
use std::sync::Mutex;
use std::rc::Rc;

fn main() {
    let join_handle = thread::spawn(move || {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1))
        }
    });
    println!("Hello, world!");

    // join method will let the  spawned_thread to complete  its task
    join_handle.join().unwrap();

    //-----------------------------use move in closure wrt to thread implementationS ------------------------

    let v = vec![8, 6, 7, 8, 5];

    let handle_vector = thread::spawn(move || {
        // here we used move
        // may be we need to drop v variable for some other logic
        // so we take consideration in moving ownership of vector
        //  because of this thread dependency

        println!("Here  the vector {:?}", v)
    });

    handle_vector.join().unwrap();

    //--------------------------------------------------------------------------

    // thread==> these are the computing unit that let run the independent code.

    // -----------------------------CHANNEL-------------------------------------------

    // these are the kind of connection through which two thread talk to each other.
    //channel have two parts one is transmitter and second one is receiver
    // transmitter source of data
    // receiver= that get data from transmitter

    // to create channel we will use the "mpsc" standard library also known as multiple producer

    let (tx,rx) = mpsc::channel();

    // move transmission into new spawned thread and try to communicate with the main thread by sending some data

    // thread::spawn(move ||{
    //     let val = String::from("hi from transmitter !");
    //     tx.send(val).unwrap(); // so this will also move the "val" variable
    // });

    // // now receive the data in main thread

    // // as rx.recv() receive the value transfer by the transmitter

    // let received = rx.recv().unwrap();
    // println!("got: {}", received);

    //-------------------------------------------------------------------
    let tx1 = tx.clone(); 
    // here this will create one extra transmitter
    // which we use in another spawned thread
    thread::spawn(move || {
        let values = vec![
            String::from("hi"),
            String::from("there"),
            String::from("deepak "),
            String::from("side"),
        ];

        for val in values {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    thread::spawn(move || {
        let values = vec![
            String::from("hi again"),
            String::from("there again"),
            String::from("deepak again"),
            String::from("this side again"),
        ];

        for val in values {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx{
        println!(" msgs from transmitter {}", received);
    }

    

     










}
