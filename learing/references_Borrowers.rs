fn main() {
    // let s1 = String::from("abc");
    do_stuff(&s1);
    println!("{}", s1);

    // here whenever the new reference is created is is always imutable
    let mut s1 = String::from("ABC");
    do_stuff1(&mut s1);
}
//here s1 and &s1 difference is s1 will give you value but
// &s1 will return reference of that value in side it

fn do_stuff(s: &String) {}

fn do_stuff1(s: &mut String) {
    s.insert_str(0, "Hi, ");
    // if we dont deference the value then is hase to be use like
    (*s).insert_str(0, "hi,");

    // for now the insert_str followed by . operator is dereferencing the referceing value
    // so in other case we complsary has to use *s

    //e.g
    *s = String::from("ABC");
}
// for now// lets make e.g x

// x value
// &x immutable reference
// &mut x mutable reference

// i32 is type
// &i32 is the immutable type of your reference
// &mut i32 the mutable type of your reference

// if x: &mut i32
// *x // gives a mutable i32 (deferenceing)

//x:i32
//*x  gives a immutable i32 (deferenceing)


fn main() {
    let mut s1 = String::from("abs");
    s1 = do_stuff(s1);
    println!("{}", s1);
}

// fn do_stuff(s: String) {}
// this will give error due to s1 now moved to this function and origin state is blank

fn do_stuff(s: String) -> String {
    s
}


//it works but not as we wanted 
// due to which we use References and Borrowing