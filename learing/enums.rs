enum Color {
    Red,
    Green,
    Blue,
}

fn main() {
    let color = Color::Red;

    use DispenserItem::*;
    let item = Things("hat".to_String);
    let item2 = Place { x: 3, y: 3 };

    if let Some(x) = my_variable {
        println!("value is {}", x);
    }

    match my_variable {
        Some() => {
            println!("value is {}", x);
        }
        None => {
            println!("no value");
        }
    }

    //another method

    match my_variable {
        Some(x) => x.squared() + 1,
        None => 42,
    }

    let x = match my_variable {
        Some(x) => x.squared() + 1,
        None => 42,
    };

    //Option & Result
    // special enums
}

enum DispenserItem {
    Empty,
    Ammo(u8),                 //annomras data
    Things(String, i32),      // tuple
    Place { x: i32, y: i32 }, // struct data
}

impl DispenserItem {
    fn display(&self) {}
}

//options

enum Option<T> {
    Some(T),
    None,
}

//T :any type

fn main() {
    //     let mut x: Option<i32> = None;
    //     x = Some(5);
    // }

    let mut x = None;
    x = Some(3);
    x.is_some(); //true
    x.is_none(); //false

    for i in x {
        println!("{}", i); // out put 3
    }
}
#[must_use]
enum Result<T, E> {
    Ok(T),
    Err(E),
}

// use std::fs::File;

// fn main() {
//     File::open("foo");
// }
// here we dont have error handling way

// but we can do that with unwrap()

use std::fs::File;

fn main() {
    let res = File::open("foo");
    // let f = res.unwrap();

    // let f = res.expect("Error Message");

    if res.is_ok() {
        let f = res.unwrap();
    }
}

fn main() {
    let res = File::open("foo");

    match res{
        // Ok(f) =>{ do stuff },
        // Err(e) =>{ do stuff}, 
    }
}
