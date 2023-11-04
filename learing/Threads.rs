use std::thread;

fn main() {
    let handle = thread::spawn(move || {
        //do stuff in a  child thread
    });

    // fo stuff simultaneously in the main thread

    //wait until thread has exited
    handl.join().unwrap();
}
