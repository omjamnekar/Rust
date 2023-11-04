// main rules for ownerShip

// 1. Each value has an owner
// 2. Only one owner
// 3. Values gets dropped if its owner goes out of scope

fn main() {
    // let s1 = String::from("abc");
    // let s2 = s1;
    // println!("{}", s1);

    let s1 = String::from("ABV");

    let s2 = s1.clone();
    println!("{}", s1);
    println!("{}", s2);

    //here the memory pointer is store in stack is locating the data to the heap
    // onces the variable is copied the stack get copied and due to the maikn rule of rust the memory dublicate wont exist
    // which leeds  to the original variable vanishing
    // clone is the same but when the variable get copied the values heap is get duplicate

    //here  those all three rules are satisfied
}
