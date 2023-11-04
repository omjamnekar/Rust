

pub fn greet(){
    println!("hi!!");
}


use hello::greet;

// fn main(){
//     hello::greet();    
// }

fn main(){
    greet();
}

use std::collection::HashMap;


use rand::thread_rng;


fn main(){
    let x=thread_rng().gen_range(0,10);
}


// fn main(){
//     let x= rand::thread_rng().gen_range(0,100);
//     println!("{}".x);
// }